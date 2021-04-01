(function() {var implementors = {};
implementors["salvo_core"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html\" title=\"enum core::convert::Infallible\">Infallible</a>&gt; for <a class=\"struct\" href=\"salvo_core/struct.Error.html\" title=\"struct salvo_core::Error\">Error</a>","synthetic":false,"types":["salvo_core::error::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/httparse/1.3.5/httparse/enum.Error.html\" title=\"enum httparse::Error\">Error</a>&gt; for <a class=\"enum\" href=\"salvo_core/http/errors/read_error/enum.ReadError.html\" title=\"enum salvo_core::http::errors::read_error::ReadError\">ReadError</a>","synthetic":false,"types":["salvo_core::http::errors::read_error::ReadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"salvo_core/http/errors/read_error/enum.ReadError.html\" title=\"enum salvo_core::http::errors::read_error::ReadError\">ReadError</a>","synthetic":false,"types":["salvo_core::http::errors::read_error::ReadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"salvo_core/http/errors/read_error/enum.ReadError.html\" title=\"enum salvo_core::http::errors::read_error::ReadError\">ReadError</a>","synthetic":false,"types":["salvo_core::http::errors::read_error::ReadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/str/error/struct.Utf8Error.html\" title=\"struct core::str::error::Utf8Error\">Utf8Error</a>&gt; for <a class=\"enum\" href=\"salvo_core/http/errors/read_error/enum.ReadError.html\" title=\"enum salvo_core::http::errors::read_error::ReadError\">ReadError</a>","synthetic":false,"types":["salvo_core::http::errors::read_error::ReadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.64/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"salvo_core/http/errors/read_error/enum.ReadError.html\" title=\"enum salvo_core::http::errors::read_error::ReadError\">ReadError</a>","synthetic":false,"types":["salvo_core::http::errors::read_error::ReadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"salvo_core/http/request/struct.Body.html\" title=\"struct salvo_core::http::request::Body\">Body</a>&gt; for <a class=\"enum\" href=\"salvo_core/http/response/enum.Body.html\" title=\"enum salvo_core::http::response::Body\">Body</a>","synthetic":false,"types":["salvo_core::http::response::Body"]}];
implementors["salvo_extra"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;DecodeError&gt; for <a class=\"enum\" href=\"salvo_extra/auth/basic/enum.Error.html\" title=\"enum salvo_extra::auth::basic::Error\">Error</a>","synthetic":false,"types":["salvo_extra::auth::basic::Error"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()