use axum::extract::ws as ax_ws;
use axum::{
    extract::{WebSocketUpgrade, State},
    response::{IntoResponse, Html},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use serde::Serialize;
use crate::vision::{VisionPipeline, RecognitionResult};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct VisionResponse {
    timestamp: u64,
    recognition: RecognitionResult,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

struct AppState {
    pipeline: VisionPipeline,
}

pub async fn run_server() {
    let state = Arc::new(AppState {
        pipeline: VisionPipeline::new(),
    });

    let app = Router::new()
        .route("/ws/vision", get(ws_handler))
        .fallback_service(
            ServeDir::new("frontend/dist").not_found_service(get(|| async {
                Html(include_str!("../frontend/dist/index.html"))
            })),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: ax_ws::WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            ax_ws::Message::Binary(data) => {
                match state.pipeline.process_frame(&data).await {
                    Ok(result) => {
                        let response = VisionResponse {
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                            recognition: result,
                        };
                        let json = serde_json::to_string(&response).unwrap();
                        if sender.send(ax_ws::Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let err_resp = ErrorResponse { error: e };
                        let json = serde_json::to_string(&err_resp).unwrap();
                        if sender.send(ax_ws::Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
            ax_ws::Message::Close(_) => break,
            _ => (),
        }
    }
}
