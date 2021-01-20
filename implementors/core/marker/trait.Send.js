(function() {var implementors = {};
implementors["salvo_core"] = [{"text":"impl Send for CatcherImpl","synthetic":true,"types":[]},{"text":"impl Send for Depot","synthetic":true,"types":[]},{"text":"impl Send for HttpError","synthetic":true,"types":[]},{"text":"impl Send for ReadError","synthetic":true,"types":[]},{"text":"impl Send for FormData","synthetic":true,"types":[]},{"text":"impl Send for FilePart","synthetic":true,"types":[]},{"text":"impl Send for FieldHeaders","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; Send for NextField&lt;'a, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; Send for Field&lt;'a, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; Send for FieldData&lt;'a, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Send for ReadToString&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Send for Multipart&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for HttpRange","synthetic":true,"types":[]},{"text":"impl Send for Request","synthetic":true,"types":[]},{"text":"impl Send for ResponseBody","synthetic":true,"types":[]},{"text":"impl Send for Response","synthetic":true,"types":[]},{"text":"impl&lt;F&gt; Send for FnFilter&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Router","synthetic":true,"types":[]},{"text":"impl Send for DetectMatched","synthetic":true,"types":[]},{"text":"impl Send for PathState","synthetic":true,"types":[]},{"text":"impl Send for Server","synthetic":true,"types":[]},{"text":"impl Send for TlsServer","synthetic":true,"types":[]},{"text":"impl Send for HyperHandler","synthetic":true,"types":[]},{"text":"impl Send for NamedFile","synthetic":true,"types":[]},{"text":"impl Send for NamedFileBuilder","synthetic":true,"types":[]},{"text":"impl Send for FileChunk","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for HtmlTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for JsonTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for PlainTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for XmlTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Send for Error","synthetic":true,"types":[]}];
implementors["salvo_extra"] = [{"text":"impl Send for BasicAuthHandler","synthetic":true,"types":[]},{"text":"impl Send for BasicAuthConfig","synthetic":true,"types":[]},{"text":"impl&lt;C&gt; Send for JwtHandler&lt;C&gt;","synthetic":true,"types":[]},{"text":"impl&lt;C&gt; Send for JwtConfig&lt;C&gt;","synthetic":true,"types":[]},{"text":"impl Send for HeaderExtractor","synthetic":true,"types":[]},{"text":"impl Send for FormExtractor","synthetic":true,"types":[]},{"text":"impl Send for QueryExtractor","synthetic":true,"types":[]},{"text":"impl Send for CookieExtractor","synthetic":true,"types":[]},{"text":"impl Send for Error","synthetic":true,"types":[]},{"text":"impl Send for Options","synthetic":true,"types":[]},{"text":"impl Send for Static","synthetic":true,"types":[]},{"text":"impl Send for Builder","synthetic":true,"types":[]},{"text":"impl Send for CorsHandler","synthetic":true,"types":[]},{"text":"impl Send for WsHandler","synthetic":true,"types":[]},{"text":"impl Send for WebSocket","synthetic":true,"types":[]},{"text":"impl Send for Message","synthetic":true,"types":[]},{"text":"impl Send for MissingConnectionUpgrade","synthetic":true,"types":[]},{"text":"impl Send for SseEvent","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Send for SseKeepAlive&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()