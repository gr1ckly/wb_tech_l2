use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::error::Error;
use std::sync::{Arc};
use axum::extract::{OriginalUri, Request, State};
use axum::http::{StatusCode};
use axum::{debug_handler, Json, Router};
use axum::body::Bytes;
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use chrono::{DateTime, Utc};
use log::info;
use serde_json;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use crate::model::{Event, Model};

const BAD_PARAM_MESSAGE: &str = "The curr_time must be passed as a single parameter in the format: YYYY-mm-DDTHH:MM:SSZ";
const INVALID_JSON_MESSAGE: &str = "Failed to recognize JSON";
const BAD_CREATE_EVENT_MESSAGE: &str = "The request must only contain the following fields: start_time and end_time in the format YYYY-mm-DDTHH:MM:SSZ, name - a string value of up to 255 characters and description - a string value of up to 1000 characters.";
const BAD_UPDATE_EVENT_MESSAGE: &str = "The request must only contain the following fields: id - an integer value greater 0,  start_time and end_time in the format YYYY-mm-DDTHH:MM:SSZ, name - a string value of up to 255 characters and description - a string value of up to 1000 characters.";
const BAD_DELETE_EVENT_MESSAGE: &str = "The request must only contain the id - an integer value greater 1.";

struct AppState{
    model: Model,
}

async fn log_middleware(request: Request, next: Next) -> Result<Response, Infallible> {
    let uri = request.uri();
    let method = request.method();
    let headers = request.headers();
    info!("Request uri: {}, method: {}, headers: {}", uri.to_string(), method.to_string(), headers.iter().map(|(k, v)| {
        (k.to_string(), v.to_str().unwrap().to_string())
    }).collect::<serde_json::Value>());
    Ok(next.run(request).await)
}

pub async fn launch() -> Result<(), Box<dyn Error>>{
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let model = Model::init().await?;

    let router = Router::new().route("/events_for_day", get(handler_events_for_day))
        .route("/events_for_week", get(handler_events_for_week))
        .route("/events_for_month", get(handler_events_for_month))
        .route("/create_event", post(handler_create_event))
        .route("/update_event", post(handler_update_event))
        .route("/delete_event", post(handler_delete_event))
        .layer(ServiceBuilder::new().layer(from_fn(log_middleware)))
        .with_state(Arc::new(RwLock::new(AppState {
            model: model
        })));

    let server_host = env::var("SERVER_HOST").expect("Couldn't get SERVER_HOST");
    let server_port = env::var("SERVER_PORT").expect("Couldn't get SERVER_PORT").parse::<u16>().expect("Invalid format SERVER_PORT");

    let tcp_listener = TcpListener::bind(format!("{}:{}", server_host, server_port)).await.expect("Couldn't create TcpListener");

    axum::serve(tcp_listener, router).await?;

    Ok(())
}

async fn handler_create_event(State(state): State<Arc<RwLock<AppState>>>, body: Bytes) -> impl IntoResponse {
    match serde_json::from_slice::<Value>(&body) {
        Ok(values) => {
            if count_value_elements(&values) == 4 {
                if let Some(event) = extract_event(values) {
                    let model = state.read().await;
                    return match model.model.add_event(event).await {
                        Ok(res) => (StatusCode::OK, Json(json!({"result": res}))),
                        Err(e) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error": e.to_string()}))),
                    };
                }
            }
            (StatusCode::BAD_REQUEST, Json(json!({"error": BAD_CREATE_EVENT_MESSAGE})))
        }
        Err(_) => (StatusCode::BAD_REQUEST, Json(json!({"error": INVALID_JSON_MESSAGE})))
    }
}


async fn handler_update_event(State(state): State<Arc<RwLock<AppState>>>, body: Bytes) -> impl IntoResponse{
    match serde_json::from_slice::<Value>(&body){
        Ok(values) => {
            if count_value_elements(&values) == 5{
                if let Some(event) = extract_event(values){
                    if event.get_id() > &0{
                        let model = state.read().await;
                        return match model.model.update_event(event).await{
                            Ok(res) => (StatusCode::OK, Json(json!({"result": res}))),
                            Err(e) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error": e.to_string()})))
                        };
                    }
                }
            }
            (StatusCode::BAD_REQUEST, Json(json!({"error": BAD_UPDATE_EVENT_MESSAGE})))
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": INVALID_JSON_MESSAGE})))
    }
}

async fn handler_delete_event(State(state): State<Arc<RwLock<AppState>>>, body: Bytes) -> impl IntoResponse{
    match serde_json::from_slice::<Value>(&body){
        Ok(values) => {
            if count_value_elements(&values) == 1{
                if let Some(id) = extract_id(values){
                    let model = state.read().await;
                    return match model.model.delete_event(&id).await{
                        Ok(res) => (StatusCode::OK, Json(json!({"result": res}))),
                        Err(e) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error": e.to_string()})))
                    };
                }
            }
            (StatusCode::BAD_REQUEST, Json(json!({"error": BAD_DELETE_EVENT_MESSAGE})))
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": INVALID_JSON_MESSAGE})))
    }
}

fn count_value_elements(values: &Value) -> usize{
    match values{
        Value::Object(map) => map.len(),
        Value::Array(arr) => arr.len(),
        _ => 1
    }
}

fn extract_event(values: Value) -> Option<Event> {
    match serde_json::from_value::<Event>(values){
        Ok(event) => {
            if event.get_name().len() > 255 || event.get_description().len() > 1000{
                return None;
            }
            Some(event)
        },
        Err(e) => None,
    }
}

fn extract_id(values: Value) -> Option<i64>{
    if let Some(id) = values["id"].as_i64(){
        if id > 0{
            return Some(id)
        }
    }
    None
}

async fn handler_events_for_day(uri: OriginalUri, State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    match extract_param(uri).await{
        Ok(time) => {
            let mut model = state.read().await;
            let ans_vec = model.model.get_events_for_day(time).await;
            match ans_vec{
                Ok(events) => (StatusCode::OK, Json(json!({"result": events}))),
                Err(err) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error": err.to_string()})))
            }
        },
        Err(()) => (StatusCode::BAD_REQUEST, Json(json!({"error": BAD_PARAM_MESSAGE})))
    }
}

async fn handler_events_for_week(uri: OriginalUri, State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    match extract_param(uri).await{
        Ok(time) => {
            let mut model = state.read().await;
            let ans_vec = model.model.get_events_for_week(time).await;
            match ans_vec{
                Ok(events) => (StatusCode::OK, Json(json!({"result": events}))),
                Err(err) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error": err.to_string()})))
            }
        },
        Err(()) => (StatusCode::BAD_REQUEST, Json(json!({"error": BAD_PARAM_MESSAGE})))
    }
}

async fn handler_events_for_month(uri: OriginalUri, State(state): State<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    match extract_param(uri).await{
        Ok(time) => {
            let mut model = state.read().await;
            let ans_vec = model.model.get_events_for_month(time).await;
            match ans_vec{
                Ok(events) => (StatusCode::OK, Json(json!({"result": events}))),
                Err(err) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"error": err.to_string()})))
            }
        },
        Err(()) => (StatusCode::BAD_REQUEST, Json(json!({"error": BAD_PARAM_MESSAGE})))
    }
}

async fn extract_param(uri: OriginalUri) -> Result<DateTime<Utc>, ()>{
    if let Some(query) = uri.query() {
        let params: HashMap<&str, &str> = query.trim().split("&").filter_map(|pair| {
            let mut split = pair.splitn(2, "=");
            Some((split.next()?, split.next()?))
        }).collect();
        if params.len() == 1{
            if let Some(value) = params.get("curr_time"){
                if let Ok(time) = DateTime::parse_from_rfc3339(value){
                    return Ok(time.with_timezone(&Utc));
                }
            }
        }
    }
    Err(())
}