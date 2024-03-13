Template for simple HTTP server with active websocket connection.  
Minimum dependency count, without tokio.  
Rust backend + javascript to update html page.  

Dependencies: 
 - chrono - accurate timestamps,
 - env_logger and log - logging,
 - serde_json - creation of messages sent through websocket,
 - tiny_http - http server,
 - tungstenite - websocket handler,
