renderer = PIXI.autoDetectRenderer(
  window.innerWidth, window.innerHeight,
  {antialias: true, transparent: false, resolution: 1.0}
);
document.body.appendChild(renderer.view);

renderer.view.style.position = "absolute";
renderer.view.style.display = "block";
renderer.autoResize = true;
renderer.resize(window.innerWidth, window.innerHeight);

var stage = new PIXI.Container();

PIXI.loader
  .add("images/robot.jpeg")
  .load(setup);

var robot = {};

function gameLoop() {

  //Loop this function at 60 frames per second
  requestAnimationFrame(gameLoop);

  //Render the stage to see the animation
  renderer.render(stage);
}

function setup() {
  robot = new PIXI.Sprite(
    PIXI.loader.resources["images/robot.jpeg"].texture
  );
  console.log("robot image loaded");
  robot.scale.set(0.1,0.1);
  robot.crossOrigin = '';
  stage.addChild(robot);
  gameLoop();
}

var socket = new WebSocket("ws://127.0.0.1:2794", "rust-websocket");
socket.onmessage = function (event) {
    if (event.data==="Hello") {
        console.log("connected to server");
        setInterval(function f() {send("w");}, 16);
    } else {
        xy = JSON.parse(event.data);
        robot.position.set(xy.x % window.innerWidth, xy.y % window.innerWidth);
        //console.log("received: "+event.data);

    }
};

function send(txt) {
    socket.send(txt);
}

var up = false;
var down = false;
var left = false;
var right = false;

window.onkeydown = function(e) {
    if (e.keyCode==40) {
        if (up) {
            return;
        }
        up = true;
        send("i:up_pressed")
    } else
    if (e.keyCode==38) {
        if (down) {
            return;
        }
        down = true;
        send("i:down_pressed")
    } else
    if (e.keyCode==37) {
        if (left) {
            return;
        }
        left = true;
        send("i:left_pressed")
    } else
    if (e.keyCode==39) {
        if (right) {
            return;
        }
        right = true;
        send("i:right_pressed")
    }
}
window.onkeyup = function(e) {
    if (e.keyCode==40) {
        if (!up) {
            return;
        }
        up = false;
        send("i:up_released")
    } else
    if (e.keyCode==38) {
        if (!down) {
            return;
        }
        down = false;
        send("i:down_released")
    } else
    if (e.keyCode==37) {
        if (!left) {
            return;
        }
        left = false;
        send("i:left_released")
    } else
    if (e.keyCode==39) {
        if (!right) {
            return;
        }
        right = false;
        send("i:right_released")
    }
}
