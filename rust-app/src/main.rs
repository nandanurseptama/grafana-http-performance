use actix_web::{App, HttpResponse, HttpServer, get, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct BaseBody<T> {
    message: String,
    data: Option<T>,
}

#[derive(Deserialize)]
struct FibonacciParam {
    n: u32,
}

#[get("/")]
async fn hello() -> HttpResponse {
    let body = BaseBody::<u128> {
        message: "OK".to_string(),
        data: None,
    };
    return HttpResponse::BadRequest().json(body);
}

#[get("/fibonacci")]
async fn fib(query: web::Query<FibonacciParam>) -> HttpResponse {
    if query.n < 1 {
        let body = BaseBody::<u32> {
            message: "n must be start from 1".to_string(),
            data: None,
        };
        return HttpResponse::BadRequest().json(body);
    }

    let x = fibonacci(query.n);
    let body = BaseBody::<u32> {
        message: "OK".to_string(),
        data: Some(x),
    };

    return HttpResponse::BadRequest().json(body);
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("hello");
    HttpServer::new(|| App::new().service(fib).service(hello))
        .bind(("127.0.0.1", 8083))?
        .run()
        .await
}
