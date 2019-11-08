var socket = new WebSocket("ws://localhost:3000/ws/");

// Connection opened
socket.addEventListener("open", function(event) {
  socket.send("Hello Server!");
});

// Listen for messages
socket.addEventListener("message", function(event) {
  try {
    handler(JSON.parse(event.data));
  } catch (error) {
    console.log(error);
    console.log("NO PARSEA");
    console.log("Message from server ", event.data);
  }
});

const root = document.querySelector("#root");

const app = document.createElement("main");
app.setAttribute("class", "container");

const text = document.createTextNode("Connected");

const list = document.createElement("ul");
list.setAttribute("id", "client-list");

const input = document.createElement("input");
input.setAttribute("id", "sender");

const elements = [text, list, input];

elements.forEach(element => {
  app.appendChild(element);
});

const addElementToList = element => {
  const item = document.createElement("li");
  item.setAttribute("id", element);
  const value = document.createTextNode(element);
  item.appendChild(value);
  document.querySelector("#client-list").appendChild(item);
};

const delElementFromList = element => {
  const item = document.querySelector(`#${element}`);
  const list = document.querySelector("#client-list");
  list.removeChild(item);
};

const handler = ({ type, payload }) => {
  switch (type) {
    case "ADD":
      return addElementToList(payload);

    case "DEL":
      return delElementFromList(payload);

    case "WELCOME":
      return payload.forEach(addElementToList);

    default:
      break;
  }
};

root.append(app);
