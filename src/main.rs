use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {}

fn main() {
    println!("Hello, world!");
}
