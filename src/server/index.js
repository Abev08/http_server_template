const ws = new WebSocket("ws://" + window.location.hostname + ":40001");
let conn_err;
let content;
let text;

function loaded() {
  conn_err = document.getElementById("conn_err");
  content = document.getElementById("content");

  let text2 = document.createTextNode("Number of seconds since connected: ");
  content.appendChild(text2);
  text = document.createTextNode("0");
  content.appendChild(text);
}

window.addEventListener("load", loaded);

ws.addEventListener("open", () => {
  console.log("WebSocket connection established!");
  conn_err.hidden = true;
  content.hidden = false;
})

ws.addEventListener("close", () => {
  console.log("WebSocket connection closed!");
  conn_err.hidden = false;
  content.hidden = true;
});

ws.addEventListener("message", e => {
  let data = JSON.parse(e.data);
  // console.log(data);

  // do something based on received data
  text.textContent = data.time_since_connected.secs;
});

