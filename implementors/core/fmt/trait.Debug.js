(function() {var implementors = {};
implementors["salvo_core"] = [{"text":"impl Debug for HttpError","synthetic":false,"types":[]},{"text":"impl Debug for ReadError","synthetic":false,"types":[]},{"text":"impl Debug for FilePart","synthetic":false,"types":[]},{"text":"impl Debug for FieldHeaders","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;TryStream&gt; Debug for Field&lt;'_, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S::Error: Into&lt;ReadError&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Debug for HttpRange","synthetic":false,"types":[]},{"text":"impl Debug for Request","synthetic":false,"types":[]},{"text":"impl Debug for Response","synthetic":false,"types":[]},{"text":"impl Debug for PathState","synthetic":false,"types":[]},{"text":"impl Debug for NamedFile","synthetic":false,"types":[]},{"text":"impl Debug for Error","synthetic":false,"types":[]}];
implementors["salvo_extra"] = [{"text":"impl Debug for Error","synthetic":false,"types":[]},{"text":"impl Debug for Options","synthetic":false,"types":[]},{"text":"impl Debug for Cors","synthetic":false,"types":[]},{"text":"impl Debug for Builder","synthetic":false,"types":[]},{"text":"impl Debug for WebSocket","synthetic":false,"types":[]},{"text":"impl Debug for Message","synthetic":false,"types":[]},{"text":"impl Debug for MissingConnectionUpgrade","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()