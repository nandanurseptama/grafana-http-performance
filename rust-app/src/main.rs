mod middleware;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use middleware::Metrics;
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

    return HttpResponse::Ok().json(body);
}

async fn metrics() -> impl Responder {
    use prometheus::{Encoder, TextEncoder};

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Metrics)
            .service(hello)
            .service(fib)
            .route("/metrics", web::get().to(metrics))
    })
    .bind(("0.0.0.0", 8083))?
    .run()
    .await
}
