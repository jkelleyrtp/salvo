(function() {var implementors = {};
implementors["salvo_core"] = [{"text":"impl Stream for <a class=\"struct\" href=\"salvo_core/fs/struct.FileChunk.html\" title=\"struct salvo_core::fs::FileChunk\">FileChunk</a>","synthetic":false,"types":["salvo_core::fs::FileChunk"]},{"text":"impl&lt;S:&nbsp;TryStream&gt; Stream for <a class=\"struct\" href=\"salvo_core/http/multipart/struct.FieldData.html\" title=\"struct salvo_core::http::multipart::FieldData\">FieldData</a>&lt;'_, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Ok: <a class=\"trait\" href=\"salvo_core/http/body_chunk/trait.BodyChunk.html\" title=\"trait salvo_core::http::body_chunk::BodyChunk\">BodyChunk</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Error: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"salvo_core/http/errors/read_error/enum.ReadError.html\" title=\"enum salvo_core::http::errors::read_error::ReadError\">ReadError</a>&gt;,&nbsp;</span>","synthetic":false,"types":["salvo_core::http::multipart::field::FieldData"]},{"text":"impl Stream for <a class=\"enum\" href=\"salvo_core/http/enum.ResponseBody.html\" title=\"enum salvo_core::http::ResponseBody\">ResponseBody</a>","synthetic":false,"types":["salvo_core::http::response::ResponseBody"]}];
implementors["salvo_extra"] = [{"text":"impl Stream for <a class=\"struct\" href=\"salvo_extra/ws/struct.WebSocket.html\" title=\"struct salvo_extra::ws::WebSocket\">WebSocket</a>","synthetic":false,"types":["salvo_extra::ws::WebSocket"]},{"text":"impl&lt;S&gt; Stream for <a class=\"struct\" href=\"salvo_extra/sse/struct.SseKeepAlive.html\" title=\"struct salvo_extra::sse::SseKeepAlive\">SseKeepAlive</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: TryStream&lt;Ok = <a class=\"struct\" href=\"salvo_extra/sse/struct.SseEvent.html\" title=\"struct salvo_extra::sse::SseEvent\">SseEvent</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Error: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">StdError</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,&nbsp;</span>","synthetic":false,"types":["salvo_extra::sse::SseKeepAlive"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()