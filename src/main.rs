use actix::{Actor, Addr};
use axum::{Json, Router, extract::State, routing::get};
use axum_actix::counter::{self, Counter};
use serde::Serialize;

#[actix::main]
async fn main() {
    let counter = Counter::default().start();
    let state = AppState { counter };

    let app = Router::new()
        .route("/", get(index))
        .route("/count", get(get_count))
        .route("/incr-count", get(incr_count))
        .route("/decr-count", get(decr_count))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server is listening on {}...", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    counter: Addr<Counter>,
}

#[derive(Serialize)]
struct GetCounter {
    count: usize,
}

async fn index() -> String {
    String::from("Hello, world!")
}

async fn get_count(State(state): State<AppState>) -> Json<GetCounter> {
    let count = state.counter.send(counter::Get).await.unwrap();
    Json(GetCounter { count })
}

async fn incr_count(State(state): State<AppState>) {
    state.counter.do_send(counter::Increment);
}

async fn decr_count(State(state): State<AppState>) {
    state.counter.do_send(counter::Decrement);
}
