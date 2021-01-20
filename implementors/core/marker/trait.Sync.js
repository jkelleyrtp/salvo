(function() {var implementors = {};
implementors["salvo_core"] = [{"text":"impl Sync for CatcherImpl","synthetic":true,"types":[]},{"text":"impl !Sync for Depot","synthetic":true,"types":[]},{"text":"impl Sync for HttpError","synthetic":true,"types":[]},{"text":"impl Sync for ReadError","synthetic":true,"types":[]},{"text":"impl Sync for FormData","synthetic":true,"types":[]},{"text":"impl Sync for FilePart","synthetic":true,"types":[]},{"text":"impl Sync for FieldHeaders","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; Sync for NextField&lt;'a, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; Sync for Field&lt;'a, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, S&gt; Sync for FieldData&lt;'a, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Sync for ReadToString&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Sync for Multipart&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;S as TryStream&gt;::Ok: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for HttpRange","synthetic":true,"types":[]},{"text":"impl Sync for Request","synthetic":true,"types":[]},{"text":"impl !Sync for ResponseBody","synthetic":true,"types":[]},{"text":"impl !Sync for Response","synthetic":true,"types":[]},{"text":"impl&lt;F&gt; Sync for FnFilter&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for Router","synthetic":true,"types":[]},{"text":"impl Sync for DetectMatched","synthetic":true,"types":[]},{"text":"impl Sync for PathState","synthetic":true,"types":[]},{"text":"impl Sync for Server","synthetic":true,"types":[]},{"text":"impl Sync for TlsServer","synthetic":true,"types":[]},{"text":"impl Sync for HyperHandler","synthetic":true,"types":[]},{"text":"impl Sync for NamedFile","synthetic":true,"types":[]},{"text":"impl Sync for NamedFileBuilder","synthetic":true,"types":[]},{"text":"impl Sync for FileChunk","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for HtmlTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for JsonTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for PlainTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Sync for XmlTextContent&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]}];
implementors["salvo_extra"] = [{"text":"impl Sync for BasicAuthHandler","synthetic":true,"types":[]},{"text":"impl Sync for BasicAuthConfig","synthetic":true,"types":[]},{"text":"impl&lt;C&gt; Sync for JwtHandler&lt;C&gt;","synthetic":true,"types":[]},{"text":"impl&lt;C&gt; Sync for JwtConfig&lt;C&gt;","synthetic":true,"types":[]},{"text":"impl Sync for HeaderExtractor","synthetic":true,"types":[]},{"text":"impl Sync for FormExtractor","synthetic":true,"types":[]},{"text":"impl Sync for QueryExtractor","synthetic":true,"types":[]},{"text":"impl Sync for CookieExtractor","synthetic":true,"types":[]},{"text":"impl Sync for Error","synthetic":true,"types":[]},{"text":"impl Sync for Options","synthetic":true,"types":[]},{"text":"impl Sync for Static","synthetic":true,"types":[]},{"text":"impl Sync for Builder","synthetic":true,"types":[]},{"text":"impl Sync for CorsHandler","synthetic":true,"types":[]},{"text":"impl Sync for WsHandler","synthetic":true,"types":[]},{"text":"impl !Sync for WebSocket","synthetic":true,"types":[]},{"text":"impl Sync for Message","synthetic":true,"types":[]},{"text":"impl Sync for MissingConnectionUpgrade","synthetic":true,"types":[]},{"text":"impl Sync for SseEvent","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Sync for SseKeepAlive&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()