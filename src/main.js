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
var worlds = [];

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

function interpolate_world() {
    // TODO: implement!
    //    robot.position.set(xy.x % window.innerWidth, xy.y % window.innerWidth);

    if (!time_has_started) {
        return;
    }

    t = performance.now() - t0 + world_t0;

    while ((worlds.length>1) && (worlds[1].t<t)) {
        worlds.shift();
    }

    if (worlds.length<2) {
        return;
    }

    ti = worlds[0].t;
    tf = worlds[1].t;
    dt = tf - ti;
    dti = t - ti;

    xi = worlds[0].x;
    xf = worlds[1].x;
    dx = xf - xi;

    yi = worlds[0].y;
    yf = worlds[1].y;
    dy = yf - yi;

    x = xi + dx/dt * dti;
    y = yi + dy/dt * dti;

    robot.position.set(x % window.innerWidth, y % window.innerHeight);
}

function gameLoop() {
  interpolate_world();
  requestAnimationFrame(gameLoop);
  renderer.render(stage);
}

var time_has_started = false;
var t0 = 0.0;
var world_t0 = 0.0;

var input_socket = new WebSocket("ws://127.0.0.1:2794", "input-websocket");
var world_socket = new WebSocket("ws://127.0.0.1:2794", "world-websocket");
input_socket.onmessage = function (event) {
    if (event.data==="Hello") {
        console.log("input socket connected to server");
    }
};
world_socket.onmessage = function (event) {
    if (event.data==="Hello") {
        console.log("world socket connected to server");
    } else {
        worlds.push(JSON.parse(event.data));
        if ((!time_has_started)&&(worlds.length==3)) {
            time_has_started = true;
            world_t0 = worlds[0].t;
            t0 = performance.now();
        }
    }
};

function send(txt) {
    input_socket.send(txt);
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
        send("up_pressed")
    } else
    if (e.keyCode==38) {
        if (down) {
            return;
        }
        down = true;
        send("down_pressed")
    } else
    if (e.keyCode==37) {
        if (left) {
            return;
        }
        left = true;
        send("left_pressed")
    } else
    if (e.keyCode==39) {
        if (right) {
            return;
        }
        right = true;
        send("right_pressed")
    }
}
window.onkeyup = function(e) {
    if (e.keyCode==40) {
        if (!up) {
            return;
        }
        up = false;
        send("up_released")
    } else
    if (e.keyCode==38) {
        if (!down) {
            return;
        }
        down = false;
        send("down_released")
    } else
    if (e.keyCode==37) {
        if (!left) {
            return;
        }
        left = false;
        send("left_released")
    } else
    if (e.keyCode==39) {
        if (!right) {
            return;
        }
        right = false;
        send("right_released")
    }
}
