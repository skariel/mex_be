var scene = new THREE.Scene();
var camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

var renderer = new THREE.WebGLRenderer({antialias: false, precision: "lowp"});
renderer.setSize( window.innerWidth/2, window.innerHeight/2 , false);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.BasicShadowMap;
renderer.gammaInput = true;
renderer.gammaOutput = true;
document.body.appendChild( renderer.domElement );

var geometry_box = new THREE.BoxGeometry( 1, 1, 1 );
var geometry_floor = new THREE.BoxGeometry( 5, 5, 1 );
var material_box = new THREE.MeshStandardMaterial( { color: 0xffffff, map: new THREE.TextureLoader().load("images/box_3.png") } );
var material_floor = new THREE.MeshStandardMaterial( { color: 0xffffff, map: new THREE.TextureLoader().load("images/floor_1.jpg") } );

var light = new THREE.AmbientLight( 0xffffff, 0.0 ); // soft white light

for (x=-3; x<3;x++) {
    for (y=-3; y<3;y++) {
        var cube = new THREE.Mesh( geometry_floor, material_floor );
        cube.receiveShadow = true;
        cube.castShadow = false;
        cube.position.x=x*5;
        cube.position.y=y*5;
        scene.add( cube );
    }
}
cube = new THREE.Mesh( geometry_box, material_box );
cube.position.z = 0.3;
cube.scale.x=0.3;
cube.receiveShadow = true;
cube.castShadow = true;
scene.add(cube);
cube = new THREE.Mesh( geometry_box, material_box );
cube.position.z = 0.3;
cube.position.y= 0.45;
cube.position.x=0.35;
cube.scale.y=0.3;
cube.receiveShadow = true;
cube.castShadow = true;
scene.add(cube);

cube = new THREE.Mesh( geometry_box, material_box );
cube.position.z = 0.3;
cube.position.y= -1.45;
cube.position.x=-1.35;
cube.scale.y=0.3;
cube.receiveShadow = true;
cube.castShadow = true;
scene.add(cube);

scene.add( light );



var spotLight = new THREE.PointLight( 0xffffff );
spotLight.castShadow = true;
spotLight.position.set( 2, 2, 0.55 );

spotLight.castShadow = true;
spotLight.distance = 8.0;

spotLight.shadow.mapSize.width = 512;
spotLight.shadow.mapSize.height = 512;

spotLight.shadow.camera.near = 0.2;
spotLight.shadow.camera.far = 8.0;

scene.add( spotLight );





camera.position.z = 5;
camera.position.y = -1.0;
camera.lookAt(new THREE.Vector3(0,0.5,0));

var worlds = [];

var i=0
function interpolate_world() {
    if (!time_has_started) {
        return;
    }

    t = performance.now() - t0 + world_t0;

    while ((worlds.length>2) && (worlds[1].t<(t-0.01))) {
        worlds.shift();
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

    i+=1;

    xx = (x*0.05+2.0) % window.innerWidth;
    yy = (y*0.05+2.0) % window.innerWidth
    if (i % 100 == 0) {
        console.log(xx,yy);
    }

    spotLight.position.set(xx, yy, 0.8);
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
    setTimeout(function() {
        input_socket.send(txt);    
    }, 50);
}

var up = false;
var down = false;
var left = false;
var right = false;

window.onkeydown = function(e) {
    if (e.keyCode==38) {
        if (up) {
            return;
        }
        up = true;
        send("up_pressed")
    } else
    if (e.keyCode==40) {
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
    if (e.keyCode==38) {
        if (!up) {
            return;
        }
        up = false;
        send("up_released")
    } else
    if (e.keyCode==40) {
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






var stats = new Stats();
stats.showPanel( 0 ); // 0: fps, 1: ms, 2: mb, 3+: custom
document.body.appendChild( stats.dom );

function animate() {

    stats.begin();

    // monitored code goes here
    interpolate_world();
    cube.rotation.z += 0.01;
	renderer.render( scene, camera );



    stats.end();

    requestAnimationFrame( animate );

}

requestAnimationFrame( animate );





