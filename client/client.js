var socket = new WebSocket("ws://localhost:3000/ws/");

// Connection opened
socket.addEventListener("open", function(event) {
  socket.send("Hello Server!");
});

// Listen for messages
socket.addEventListener("message", function(event) {
  console.log("Message from server ", event.data);
});

const root = document.querySelector("#root");

const app = document.createElement("main");
app.setAttribute("class", "container")

const text = document.createTextNode("Connected");

const list = document.createElement("ul")
list.setAttribute("id", "client-list")


const input = document.createElement("input");
input.setAttribute("id", "sender");

const elements = [text, list,input];

elements.forEach(element => {
  app.appendChild(element);
});

root.append(app);
