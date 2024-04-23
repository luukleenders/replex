// use futures_util::{SinkExt, StreamExt};
// use salvo::prelude::*;
// use salvo::websocket::{Message as SalvoMessage, WebSocket, WebSocketUpgrade};
// use std::borrow::Cow;
// use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
// // use tokio_tungstenite::{
// //     accept_async, client_async, connect_async, WebSocketStream,
// // };

// use crate::utils::url_from_request;

// #[handler]
// pub async fn handler(
//     req: &mut Request,
//     res: &mut Response,
// ) -> Result<(), StatusError> {
//     dbg!("websocket handler");
//     // WebSocketUpgrade::new()
//     //     .upgrade(req, res, handle_client_connection)
//     //     .await

//     // let url = url_from_request(req);
//     // // replace http:// or https:// with ws:// or wss://
//     // let ws_url = url
//     //     .as_str()
//     //     .replacen("http", "ws", 1)
//     //     .replacen("https", "wss", 1);
//     // req.extensions_mut().insert(ws_url);

//     dbg!(&req);

//     let upgrade_result = WebSocketUpgrade::new()
//         .upgrade(req, res, |ws| async move {
//             dbg!("WebSocket connection upgraded");
//             // Handle the WebSocket connection here
//         })
//         .await;

//     match upgrade_result {
//         Ok(_) => {
//             dbg!("WebSocket upgrade successful");
//         }
//         Err(e) => {
//             dbg!("WebSocket upgrade failed", e);
//         }
//     }

//     Ok(())
// }

// async fn handle_client_connection(client_ws: WebSocket) {
//     let (mut client_tx, mut client_rx) = client_ws.split();
//     // let upstream_url = url_from_request(req);
//     dbg!("handle_client_connection");
//     dbg!(&client_tx);
//     dbg!(&client_rx);
//     let upstream_url = "ws://echo.websocket.org";
//     // Establish connection to upstream server
//     let (upstream_ws, _) =
//         match tokio_tungstenite::connect_async(upstream_url).await {
//             Ok(ws) => ws,
//             Err(e) => {
//                 tracing::error!(
//                     "Failed to connect to upstream WebSocket server: {}",
//                     e
//                 );
//                 return;
//             }
//         };

//     let (mut upstream_tx, mut upstream_rx) = upstream_ws.split();

//     // Forward messages from client to upstream
//     let client_to_upstream = async move {
//         while let Some(result) = client_rx.next().await {
//             if let Ok(msg) = result {
//                 match convert_salvo_to_tungstenite_message(msg) {
//                     Ok(tungstenite_msg) => {
//                         if upstream_tx.send(tungstenite_msg).await.is_err() {
//                             tracing::error!(
//                                 "Failed to send message to upstream server"
//                             );
//                             break;
//                         }
//                     }
//                     Err(err) => {
//                         tracing::error!("Failed to convert message: {}", err);
//                         break;
//                     }
//                 }
//             } else {
//                 tracing::error!("Error receiving message from client");
//                 break;
//             }
//         }
//     };

//     // Forward messages from upstream to client
//     let upstream_to_client = async move {
//         while let Some(result) = upstream_rx.next().await {
//             if let Ok(msg) = result {
//                 let salvo_msg = convert_tungstenite_to_salvo_message(msg);

//                 if client_tx.send(salvo_msg).await.is_err() {
//                     tracing::error!("Failed to send message to client");
//                 }
//             } else {
//                 tracing::error!("Error receiving message from upstream");
//                 break;
//             }
//         }
//     };

//     tokio::select! {
//         _ = client_to_upstream => tracing::info!("Client to upstream link closed"),
//         _ = upstream_to_client => tracing::info!("Upstream to client link closed"),
//     }
// }

// /// Converts a `TungsteniteMessage` to a `SalvoMessage`.
// fn convert_tungstenite_to_salvo_message(
//     tungstenite_msg: TungsteniteMessage,
// ) -> SalvoMessage {
//     match tungstenite_msg {
//         TungsteniteMessage::Text(text) => SalvoMessage::text(text),
//         TungsteniteMessage::Binary(data) => SalvoMessage::binary(data),
//         TungsteniteMessage::Ping(data) => SalvoMessage::ping(data),
//         TungsteniteMessage::Pong(data) => SalvoMessage::pong(data),
//         TungsteniteMessage::Close(optional_close_frame) => {
//             optional_close_frame.map_or(SalvoMessage::close(), |frame| {
//                 // Ensure the close code is converted to u16 and reason to Cow<'static, str>
//                 let code: u16 = frame.code.into();
//                 let reason: Cow<'static, str> = Cow::Owned(frame.reason.into());
//                 SalvoMessage::close_with(code, reason)
//             })
//         }
//         _ => SalvoMessage::close(), // Handling unexpected types conservatively.
//     }
// }

// /// Converts a `SalvoMessage` to a `TungsteniteMessage`.
// fn convert_salvo_to_tungstenite_message(
//     salvo_msg: SalvoMessage,
// ) -> Result<TungsteniteMessage, &'static str> {
//     // The SalvoMessage does not provide a direct way to pattern match on its type, so we use provided methods:
//     if salvo_msg.is_text() {
//         Ok(TungsteniteMessage::Text(
//             salvo_msg
//                 .to_str()
//                 .map_err(|_| "Failed to get text")?
//                 .to_string(),
//         ))
//     } else if salvo_msg.is_binary() {
//         Ok(TungsteniteMessage::Binary(salvo_msg.into_bytes()))
//     } else if salvo_msg.is_ping() {
//         Ok(TungsteniteMessage::Ping(salvo_msg.into_bytes()))
//     } else if salvo_msg.is_pong() {
//         Ok(TungsteniteMessage::Pong(salvo_msg.into_bytes()))
//     } else if salvo_msg.is_close() {
//         Ok(TungsteniteMessage::Close(None)) // You may need to handle Close more specifically
//     } else {
//         Err("Unsupported SalvoMessage type")
//     }
// }
