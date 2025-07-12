use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;
use futures_util::{StreamExt, SinkExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsMessage {
    TicketUpdate { ticket_id: Uuid, data: String },
    TypingIndicator { ticket_id: Uuid, user_id: Uuid, is_typing: bool },
    NewComment { ticket_id: Uuid, comment: String },
}

pub struct WsState {
    pub tx: broadcast::Sender<WsMessage>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<WsState>) {
    let mut rx = state.tx.subscribe();
    
    let (mut sender, mut receiver) = socket.split();
    
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    let _ = state.tx.send(ws_msg);
                }
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => send_task.abort(),
        _ = (&mut send_task) => recv_task.abort(),
    }
} 