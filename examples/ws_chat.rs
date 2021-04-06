//port from https://github.com/seanmonstar/warp/blob/master/examples/ws_chat.rs

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use futures::{FutureExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing_subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

use salvo::prelude::*;
use salvo_extra::ws::{Message, WsHandler};

type Users = RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, salvo::Error>>>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static ONLINE_USERS: Lazy<Users> = Lazy::new(|| Users::default());

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "ws_chat=debug,salvo=debug".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let router = Router::new()
        .handle(index)
        .push(Router::new().path("chat").handle(user_connected));
    Server::new(router).bind(([0, 0, 0, 0], 7878)).await;
}

#[fn_handler]
async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), HttpError> {
    let fut = WsHandler::new().handle(req, res)?;
    let fut = async move {
        if let Some(ws) = fut.await {
            // Use a counter to assign a new unique ID for this user.
            let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

            tracing::info!("new chat user: {}", my_id);

            // Split the socket into a sender and receive of messages.
            let (user_ws_tx, mut user_ws_rx) = ws.split();

            // Use an unbounded channel to handle buffering and flushing of messages
            // to the websocket...
            let (tx, rx) = mpsc::unbounded_channel();
            let rx = UnboundedReceiverStream::new(rx);
            let fut = rx.forward(user_ws_tx).map(|result| {
                if let Err(e) = result {
                    tracing::error!(error = ?e, "websocket send error");
                }
            });
            tokio::task::spawn(fut);
            let fut = async move {
                ONLINE_USERS.write().await.insert(my_id, tx);

                while let Some(result) = user_ws_rx.next().await {
                    let msg = match result {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprintln!("websocket error(uid={}): {}", my_id, e);
                            break;
                        }
                    };
                    user_message(my_id, msg).await;
                }

                user_disconnected(my_id).await;
            };
            tokio::task::spawn(fut);
        }
    };
    tokio::task::spawn(fut);
    Ok(())
}
async fn user_message(my_id: usize, msg: Message) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in ONLINE_USERS.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: usize) {
    eprintln!("good bye user: {}", my_id);
    // Stream closed up, so remove from the user list
    ONLINE_USERS.write().await.remove(&my_id);
}

#[fn_handler]
async fn index(res: &mut Response) {
    res.render_html_text(INDEX_HTML);
}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>WS Chat</title>
    </head>
    <body>
        <h1>WS Chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="msg" />
        <button type="button" id="submit">Submit</button>
        <script>
        const chat = document.getElementById('chat');
        const msg = document.getElementById('msg');
        const submit = document.getElementById('submit');
        const ws = new WebSocket(`ws://${location.host}/chat');

        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };

        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';

            message('<You>: ' + msg);
        };
        function showMessage(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }
        </script>
    </body>
</html>
"#;
