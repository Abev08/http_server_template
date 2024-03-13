use std::{
  net::TcpListener,
  thread,
  time::{Duration, Instant},
};

use serde_json::json;
use tiny_http::{Header, Response, Server, StatusCode};
use tungstenite::Message;

const INDEX_HTML: &str = include_str!("server/index.html");
const INDEX_JS: &str = include_str!("server/index.js");

pub fn start(ip: &str) {
  let http_address = format!("{}:40000", ip);
  let websocket_address = format!("{}:40001", ip);

  // Create client thread
  thread::Builder::new()
    .name("HTTP".to_string())
    .spawn(move || update_new_clients(http_address))
    .expect("Spawning http server thread failed!");

  // Create client websocket thread
  thread::Builder::new()
    .name("WEBSOCKET".to_string())
    .spawn(move || update_websockets(websocket_address))
    .expect("Spawning websocket server thread failed!");
}

fn update_new_clients(http_address: String) {
  log::info!("HTTP server started at: http://{}", &http_address);
  let server = Server::http(http_address).unwrap();

  for request in server.incoming_requests() {
    match request.url() {
      "/" => {
        let resp = Response::from_string(INDEX_HTML).with_header(Header {
          field: "Content-Type".parse().unwrap(),
          value: "text/html; charset=UTF-8".parse().unwrap(),
        });
        request
          .respond(resp)
          .expect("Couldn't respond to the request");
      }
      "/index.js" => {
        let resp = Response::from_string(INDEX_JS);
        request
          .respond(resp)
          .expect("Couldn't respond to the request");
      }
      _ => {
        let response = Response::new_empty(StatusCode(204));
        request
          .respond(response)
          .expect("Couldn't respond to the request");
      }
    }
  }
}

fn update_websockets(websocket_address: String) {
  log::info!("Websocket server start");

  let listener = TcpListener::bind(websocket_address).unwrap();
  let read_timeout = Some(Duration::from_millis(10));

  for stream in listener.incoming() {
    thread::spawn(move || {
      let mut socket = tungstenite::accept(stream.unwrap()).unwrap();
      socket
        .get_mut()
        .set_read_timeout(read_timeout)
        .expect("Couldn't set read timeout for the websocket!");

      let address = socket.get_ref().peer_addr().unwrap();
      log::info!("New connection: {}", address);

      let connected_at = Instant::now();
      let mut send_timer = Instant::now();
      let send_interval = Duration::from_millis(1000);

      loop {
        // Send messages
        if send_timer.elapsed() >= send_interval {
          send_timer = Instant::now();
          match socket.send(Message::Text(
            json!({
              "time_since_connected": connected_at.elapsed(),
            })
            .to_string(),
          )) {
            Err(err) => log::error!("Error while sending data to {}: {}", address, err),
            _ => {}
          }
        }

        // Read messages
        let res = socket.read();
        if res.is_err() {
          let err = res.unwrap_err();
          match err {
            tungstenite::Error::Io(_) => {
              continue;
            }
            tungstenite::Error::ConnectionClosed => {
              log::info!("Websocket connection to {} closed", address)
            }
            _ => {
              log::error!("Websocket connection to {} error: {}", address, err);
            }
          }
          return;
        }
        let msg = res.unwrap();
        println!("Message from {}, {}", address, msg);

        if msg.is_text() {
          // let text = msg.to_text().unwrap();
        }
      }
    });
  }
}
