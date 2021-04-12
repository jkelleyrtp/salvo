// Copyright (c) 2018-2020 Sean McArthur
// Licensed under the MIT license http://opensource.org/licenses/MIT
// port from https://github.com/seanmonstar/warp/blob/master/src/filters/sse.rs
//! Server-Sent Events (SSE)
//! # Example
//!
//! ```no_run
//! use std::time::Duration;
//! use std::convert::Infallible;
//! use futures::{stream::iter, Stream};
//!
//! use salvo_core::prelude::*;
//! use salvo_extra::sse::{self, SseEvent};
//!
//! fn sse_events() -> impl Stream<Item = Result<SseEvent, Infallible>> {
//!     iter(vec![
//!         Ok(SseEvent::default().data("unnamed event")),
//!         Ok(
//!             SseEvent::default().name("chat")
//!             .data("chat message")
//!         ),
//!         Ok(
//!             SseEvent::default().id(13.to_string())
//!             .name("chat")
//!             .data("other chat message\nwith next line")
//!             .retry(Duration::from_millis(5000))
//!         )
//!     ])
//! }
//! #[fn_handler]
//! async fn handle(res: &mut Response) {
//!     sse::streaming(res, sse_events());
//! }
//! #[tokio::main]
//! async fn main() {
//!     let router = Router::new().path("push-notifications").get(handle);
//!     Server::new(router).bind(([0, 0, 0, 0], 3131)).await;
//! }
//! ```
//!
//! Each field already is event which can be sent to client.
//! The events with multiple fields can be created by combining fields using tuples.
//!
//! See also the [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource) API,
//! which specifies the expected behavior of Server Sent Events.
//!

use serde::Serialize;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter, Write};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures::{future, Stream, TryStream, TryStreamExt};
use pin_project::pin_project;
use salvo_core::http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE};
use serde_json::{self, Error};
use tokio::time::{self, Sleep};

use salvo_core::http::Response;

// Server-sent event data type
#[derive(Debug)]
enum DataType {
    Text(String),
    Json(String),
}

#[derive(Debug)]
pub struct SseError;

impl Display for SseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "sse error")
    }
}

impl StdError for SseError {}
/// Server-sent event
#[derive(Default, Debug)]
pub struct SseEvent {
    name: Option<String>,
    id: Option<String>,
    data: Option<DataType>,
    comment: Option<String>,
    retry: Option<Duration>,
}

impl SseEvent {
    /// Set Server-sent event data
    /// data field(s) ("data:<content>")
    pub fn data<T: Into<String>>(mut self, data: T) -> SseEvent {
        self.data = Some(DataType::Text(data.into()));
        self
    }

    /// Set Server-sent event data
    /// data field(s) ("data:<content>")
    pub fn json_data<T: Serialize>(mut self, data: T) -> Result<SseEvent, Error> {
        self.data = Some(DataType::Json(serde_json::to_string(&data)?));
        Ok(self)
    }

    /// Set Server-sent event comment
    /// Comment field (":<comment-text>")
    pub fn comment<T: Into<String>>(mut self, comment: T) -> SseEvent {
        self.comment = Some(comment.into());
        self
    }

    /// Set Server-sent event event
    /// Event name field ("event:<event-name>")
    pub fn name<T: Into<String>>(mut self, event: T) -> SseEvent {
        self.name = Some(event.into());
        self
    }

    /// Set Server-sent event retry
    /// Retry timeout field ("retry:<timeout>")
    pub fn retry(mut self, duration: Duration) -> SseEvent {
        self.retry = Some(duration);
        self
    }

    /// Set Server-sent event id
    /// Identifier field ("id:<identifier>")
    pub fn id<T: Into<String>>(mut self, id: T) -> SseEvent {
        self.id = Some(id.into());
        self
    }
}

impl Display for SseEvent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(ref comment) = &self.comment {
            ":".fmt(f)?;
            comment.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(ref name) = &self.name {
            "event:".fmt(f)?;
            name.fmt(f)?;
            f.write_char('\n')?;
        }

        match self.data {
            Some(DataType::Text(ref data)) => {
                for line in data.split('\n') {
                    "data:".fmt(f)?;
                    line.fmt(f)?;
                    f.write_char('\n')?;
                }
            }
            Some(DataType::Json(ref data)) => {
                "data:".fmt(f)?;
                data.fmt(f)?;
                f.write_char('\n')?;
            }
            None => {}
        }

        if let Some(ref id) = &self.id {
            "id:".fmt(f)?;
            id.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(ref duration) = &self.retry {
            "retry:".fmt(f)?;

            let secs = duration.as_secs();
            let millis = duration.subsec_millis();

            if secs > 0 {
                // format seconds
                secs.fmt(f)?;

                // pad milliseconds
                if millis < 10 {
                    f.write_str("00")?;
                } else if millis < 100 {
                    f.write_char('0')?;
                }
            }

            // format milliseconds
            millis.fmt(f)?;

            f.write_char('\n')?;
        }

        f.write_char('\n')?;
        Ok(())
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct SseKeepAlive<S> {
    #[pin]
    event_stream: S,
    comment_text: Cow<'static, str>,
    max_interval: Duration,
    #[pin]
    alive_timer: Sleep,
}

impl<S> SseKeepAlive<S>
where
    S: TryStream<Ok = SseEvent> + Send + 'static,
    S::Error: StdError + Send + Sync + 'static,
{
    pub fn new(event_stream: S) -> SseKeepAlive<S> {
        let max_interval = Duration::from_secs(15);
        let alive_timer = time::sleep(max_interval);
        SseKeepAlive {
            event_stream,
            comment_text: Cow::Borrowed(""),
            max_interval,
            alive_timer,
        }
    }
    /// Customize the interval between keep-alive messages.
    ///
    /// Default is 15 seconds.
    pub fn interval(mut self, time: Duration) -> Self {
        self.max_interval = time;
        self
    }

    /// Customize the text of the keep-alive message.
    ///
    /// Default is an empty comment.
    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.comment_text = text.into();
        self
    }

    /// Send stream.
    pub fn streaming(self, res: &mut Response) {
        write_request_headers(res);
        let body_stream = self
            .map_err(|error| {
                tracing::error!("sse stream error: {}", error);
                SseError
            })
            .into_stream()
            .and_then(|event| future::ready(Ok(event.to_string())));
        res.streaming(body_stream);
    }
}
#[inline]
fn write_request_headers(res: &mut Response) {
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
    // Disable response body caching
    res.headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
}

pub fn streaming<S>(res: &mut Response, event_stream: S)
where
    S: TryStream<Ok = SseEvent> + Send + 'static,
    S::Error: StdError + Send + Sync + 'static,
{
    write_request_headers(res);
    let body_stream = event_stream
        .map_err(|error| {
            tracing::error!("sse stream error: {}", error);
            SseError
        })
        .into_stream()
        .and_then(|event| future::ready(Ok(event.to_string())));
    res.streaming(body_stream);
}

impl<S> Stream for SseKeepAlive<S>
where
    S: TryStream<Ok = SseEvent> + Send + 'static,
    S::Error: StdError + Send + Sync + 'static,
{
    type Item = Result<SseEvent, SseError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut pin = self.project();
        match pin.event_stream.try_poll_next(cx) {
            Poll::Pending => match Pin::new(&mut pin.alive_timer).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(_) => {
                    // restart timer
                    pin.alive_timer.reset(tokio::time::Instant::now() + *pin.max_interval);
                    let comment_str = pin.comment_text.clone();
                    let event = SseEvent::default().comment(comment_str);
                    Poll::Ready(Some(Ok(event)))
                }
            },
            Poll::Ready(Some(Ok(event))) => {
                // restart timer
                pin.alive_timer.reset(tokio::time::Instant::now() + *pin.max_interval);
                Poll::Ready(Some(Ok(event)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(error))) => {
                tracing::error!("sse::keep error: {}", error);
                Poll::Ready(Some(Err(SseError)))
            }
        }
    }
}
