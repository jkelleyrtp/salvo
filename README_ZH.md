<div align="center">
<img alt="Savlo" src="assets/logo.svg" />
<p>

[![build status](https://github.com/salvo-rs/salvo/workflows/CI%20(Linux)/badge.svg?branch=master&event=push)](https://github.com/salvo-rs/salvo/actions)
[![build status](https://github.com/salvo-rs/salvo//workflows/CI%20(macOS)/badge.svg?branch=master&event=push)](https://github.com/salvo-rs/salvo/actions)
[![build status](https://github.com/salvo-rs/salvo/workflows/CI%20(Windows)/badge.svg?branch=master&event=push)](https://github.com/salvo-rs/salvo/actions)
<br>
[![codecov](https://codecov.io/gh/salvo-rs/salvo/branch/master/graph/badge.svg)](https://codecov.io/gh/salvo-rs/salvo)
[![crates.io](https://img.shields.io/crates/v/salvo)](https://crates.io/crates/salvo)
[![Download](https://img.shields.io/crates/d/salvo.svg)](https://crates.io/crates/salvo)
![License](https://img.shields.io/crates/l/salvo.svg)

</p>
</div>

Salvo 是一个简单易用的 Rust Web 后端框架. 目标是让 Rust 下的 Web 后端开发能像 Go 等其他语言里的一样简单.

## 🎯 功能特色
  * 基于hyper, tokio 的异步 Web 后端框架;
  * 支持 Websocket;
  * 统一的中间件和句柄接口, 中间件系统支持在句柄之前或者之后运行;
  * 简单易用的路由系统, 支持路由嵌套, 在任何嵌套层都可以添加中间件;
  * 内置 multipart 表单处理, 处理上传文件变得非常简单;
  * 支持从多个本地目录映射成一个虚拟目录提供服务.

## ⚡️ 快速开始
你可以查看[实例代码](https://github.com/salvo-rs/salvo/tree/master/examples),  或者[访问网站](https://salvo.rs).


创建一个全新的项目:

```bash
cargo new hello_salvo --bin
```

添加依赖项到 `Cargo.toml`

```toml
[dependencies]
salvo = "0.9"
tokio = { version = "1", features = ["full"] }
```

在 `main.rs` 中创建一个简单的函数句柄, 命名为`hello_world`, 这个函数只是简单地打印文本 ```"Hello World"```.

```rust
use salvo::prelude::*;

#[fn_handler]
async fn hello_world(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render_plain_text("Hello World");
}
```

对于 fn_handler, 可以根据需求和喜好有不同种写法.

- 可以将一些没有用到的参数省略掉, 比如这里的 ```_req```, ```_depot```.

    ``` rust
    #[fn_handler]
    async fn hello_world(res: &mut Response) {
        res.render_plain_text("Hello World");
    }
    ```

- 对于任何实现 Writer 的类型都是可以直接作为函数返回值. 比如, ```&str``` 实现了 ```Writer```, 会直接按纯文本输出:

    ```rust
    #[fn_handler]
    async fn hello_world(res: &mut Response) -> &'static str {
        "Hello World"
    }
    ```

- 更常见的情况是, 我们需要通过返回一个 ```Result<T, E>``` 来简化程序中的错误处理. 如果 ```Result<T, E>``` 中 ```T``` 和 ```E``` 都实现 ```Writer```, 则 ```Result<T, E>``` 可以直接作为函数返回类型:

    ```rust
    #[fn_handler]
    async fn hello_world(res: &mut Response) -> Result<&'static str, ()> {
        Ok("Hello World")
    }
    ```

在 ```main``` 函数中, 我们需要首先创建一个根路由, 然后创建一个 Server 并且调用它的 ```bind``` 函数:

```rust
use salvo::prelude::*;

#[fn_handler]
async fn hello_world() -> &'static str {
    "Hello World"
}
#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    let server = Server::new(router);
    server.bind(([0, 0, 0, 0], 7878)).await;
}
```

### 中间件
Salvo 中的中间件其实就是 Handler, 没有其他任何特别之处.

### 树状路由系统

正常情况下我们是这样写路由的：

```rust
Router::new().path("articles").get(list_articles).post(create_article);
Router::new()
    .path("articles/<id>")
    .get(show_article)
    .patch(edit_article)
    .delete(delete_article);
```

往往查看文章和文章列表是不需要用户登录的, 但是创建, 编辑, 删除文章等需要用户登录认证权限才可以. Salvo 中支持嵌套的路由系统可以很好地满足这种需求. 我们可以把不需要用户登录的路由写到一起：

```rust
Router::new()
    .path("articles")
    .get(list_articles)
    .push(Router::new().path("<id>").get(show_article));
```

然后把需要用户登录的路由写到一起， 并且使用相应的中间件验证用户是否登录：
```rust
Router::new()
    .path("articles")
    .before(auth_check)
    .post(list_articles)
    .push(Router::new().path("<id>").patch(edit_article).delete(delete_article));
```

虽然这两个路由都有这同样的 ```path("articles")```, 然而它们依然可以被同时添加到同一个父路由, 所以最后的路由长成了这个样子:

```rust
Router::new()
    .push(
        Router::new()
            .path("articles")
            .get(list_articles)
            .push(Router::new().path("<id>").get(show_article)),
    )
    .push(
        Router::new()
            .path("articles")
            .before(auth_check)
            .post(list_articles)
            .push(Router::new().path("<id>").patch(edit_article).delete(delete_article)),
    );
```

```<id>```匹配了路径中的一个片段, 正常情况下文章的 ```id``` 只是一个数字, 这是我们可以使用正则表达式限制 ```id``` 的匹配规则, ```r"<id:/\d+/>"```. 

对于这种数字类型, 还有一种更简单的方法是使用  ```<id:num>```, 具体写法为:
- ```<id:num>```， 匹配任意多个数字字符;
- ```<id:num[10]>```， 只匹配固定特定数量的数字字符，这里的 10 代表匹配仅仅匹配 10 个数字字符;
- ```<id:num(..10)>``` 代表匹配 1 到 9 个数字字符;
- ```<id:num(3..10)>``` 代表匹配 3 到 9 个数字字符;
- ```<id:num(..=10)>``` 代表匹配 1 到 10 个数字字符;
- ```<id:num(3..=10)>``` 代表匹配 3 到 10 个数字字符;
- ```<id:num(10..)>``` 代表匹配至少 10 个数字字符.

还可以通过 ```<*>``` 或者 ```<**>``` 匹配所有剩余的路径片段. 为了代码易读性性强些, 也可以添加适合的名字, 让路径语义更清晰, 比如: ```<**file_path>```.

允许组合使用多个表达式匹配同一个路径片段, 比如 ```/articles/article_<id:num>/```.

### 文件上传
可以通过 Request 中的 get_file 异步获取上传的文件:

```rust
#[fn_handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.get_file("file").await;
    if let Some(file) = file {
        let dest = format!("temp/{}", file.filename().unwrap_or_else(|| "file".into()));
        if let Err(e) = std::fs::copy(&file.path, Path::new(&dest)) {
            res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
        } else {
            res.render_plain_text("Ok");
        }
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}
```

多文件上传也是非常容易处理的:

```rust
#[fn_handler]
async fn upload(req: &mut Request, res: &mut Response) {
    let files = req.get_files("files").await;
    if let Some(files) = files {
        let mut msgs = Vec::with_capacity(files.len());
        for file in files {
            let dest = format!("temp/{}", file.filename().unwrap_or_else(|| "file".into()));
            if let Err(e) = std::fs::copy(&file.path, Path::new(&dest)) {
                res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render_plain_text(&format!("file not found in request: {}", e.to_string()));
            } else {
                msgs.push(dest);
            }
        }
        res.render_plain_text(&format!("Files uploaded:\n\n{}", msgs.join("\n")));
    } else {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render_plain_text("file not found in request");
    }
}
```

### 更多示例
您可以从 [examples](./examples/) 文件夹下查看更多示例代码:
- [basic_auth.rs](./examples/basic_auth.rs)
- [compression.rs](./examples/compression.rs)
- [file_list.rs](./examples/file_list.rs)
- [proxy.rs](./examples/proxy.rs)
- [remote_addr.rs](./examples/remote_addr.rs)
- [routing.rs](./examples/routing.rs)
- [size_limiter.rs](./examples/size_limiter.rs)
- [sse_chat.rs](./examples/sse_chat.rs)
- [sse.rs](./examples/sse.rs)
- [tls.rs](./examples/tls.rs)
- [todos.rs](./examples/todos.rs)
- [unix_socket.rs](./examples/unix_socket.rs)
- [ws_chat.rs](./examples/ws_chat.rs)
- [ws.rs](./examples/ws.rs)

部分代码和示例移植自 [warp](https://github.com/seanmonstar/warp) and [multipart-async](https://github.com/abonander/multipart-async).

## ☕ 支持

`Salvo`是一个开源项目, 如果想支持本项目, 可以 ☕ [**在这里买一杯咖啡**](https://www.buymeacoffee.com/chrislearn). 
<p style="text-align: center;">
<img src="assets/alipay.png" alt="Alipay" width="320"/>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<img src="assets/weixin.png" alt="Weixin" width="320"/>
</p>


## ⚠️ 开源协议

Salvo 项目采用 MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
