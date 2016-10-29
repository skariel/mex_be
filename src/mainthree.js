var scene = new THREE.Scene();
var camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );

var renderer = new THREE.WebGLRenderer({antialias: false, precision: "lowp"});
renderer.setSize( window.innerWidth/1.3, window.innerHeight/1.3 , false);
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.BasicShadowMap;
renderer.gammaInput = true;
renderer.gammaOutput = true;
document.body.appendChild( renderer.domElement );

var geometry_box = new THREE.BoxGeometry( 1, 1, 1 );
var geometry_floor = new THREE.BoxGeometry( 5, 5, 1 );
var material_box = new THREE.MeshStandardMaterial( { color: 0xffffff, map: new THREE.TextureLoader().load("images/box_3.png") } );
var material_floor = new THREE.MeshStandardMaterial( { color: 0xffffff, map: new THREE.TextureLoader().load("images/floor_1.jpg") } );


var spotLight = new THREE.PointLight( 0xffffff, 3.0 );
spotLight.castShadow = true;
spotLight.position.set( 2, 2, 0.6 );

spotLight.castShadow = true;
spotLight.distance = 8.0;

spotLight.shadow.mapSize.width = 512;
spotLight.shadow.mapSize.height = 512;

spotLight.shadow.camera.near = 0.2;
spotLight.shadow.camera.far = 8.0;

scene.add( spotLight );





camera.position.z = 7;
camera.position.y = 0;
camera.lookAt(new THREE.Vector3(0,0,0));

var worlds = [];
var session_id;
var hero_key;

function interpolate_world() {
    if (!time_has_started) {
        return;
    }

    t = performance.now() - t0 + world_t0;

    while ((worlds.length>2) && (worlds[1].t<(t-0.01))) {
        worlds.shift();
    }

    var ti = worlds[0].t;
    var tf = worlds[1].t;
    var dt = tf - ti;
    var dti = t - ti;

    function interp(vi,vf) {
        var dv = vf - vi;
        return vi + dv/dt * dti;
    }

    for (var key in worlds[0].sprites) {
        var sprite_i = worlds[0].sprites[key];
        var sprite_f = worlds[1].sprites[key];
        var scene_sprite;
        if (all_scene_sprites.hasOwnProperty(key)) {
            scene_sprite = all_scene_sprites[key];
        }

        switch (sprite_i.type) {
            case "hero":
                // checking for this sessisons hero:
                if (sprite_i.key!==hero_key) {
                    // currently we don't display other heros
                    return;
                }

                // no sprite yet for hero, just the spotLight
                spotLight.position.x = interp(sprite_i.pos[0], sprite_f.pos[0]);
                spotLight.position.y = interp(sprite_i.pos[1], sprite_f.pos[1]);
                break;
            case "floor1":
                scene_sprite.position.x = interp(sprite_i.pos[0], sprite_f.pos[0]);
                scene_sprite.position.y = interp(sprite_i.pos[1], sprite_f.pos[1]);
                break;
            case "box1":
                scene_sprite.position.x = interp(sprite_i.pos[0], sprite_f.pos[0]);
                scene_sprite.position.y = interp(sprite_i.pos[1], sprite_f.pos[1]);
                scene_sprite.position.z = interp(sprite_i.pos[2], sprite_f.pos[2]);
                scene_sprite.scale.z= interp(sprite_i.scale[2], sprite_f.scale[2]);
                scene_sprite.scale.y= interp(sprite_i.scale[1], sprite_f.scale[1]);
                scene_sprite.scale.x= interp(sprite_i.scale[0], sprite_f.scale[0]);
                scene_sprite.rotation.z= interp(sprite_i.rotation[2], sprite_f.rotation[2]);
                scene_sprite.rotation.y= interp(sprite_i.rotation[1], sprite_f.rotation[1]);
                scene_sprite.rotation.x= interp(sprite_i.rotation[0], sprite_f.rotation[0]);
                break;
            default:
                break;
        }
    }
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

function clone(obj) {
    // just a shallow clone
    var copy = {}
    for (var attr in obj) {
        copy[attr] = obj[attr]
    }
    return copy;
}

var all_scene_sprites = {};

function add_new_sprite_to_scene(sprite) {
    var scene_sprite;
    // TODO: homogenize sprites in the FE
    switch (sprite.type) {
        case "hero":
            // no sprite yet for hero, just the spotLight
            scene_sprite = undefined;
            break;
        case "floor1":
            scene_sprite = new THREE.Mesh( geometry_floor, material_floor );
            scene_sprite.receiveShadow = true;
            scene_sprite.castShadow = false;
            scene_sprite.position.x=sprite.pos[0];
            scene_sprite.position.y=sprite.pos[1];
            break;
        case "box1":
            scene_sprite = new THREE.Mesh( geometry_box, material_box );
            scene_sprite.position.z = sprite.pos[2];
            scene_sprite.position.y= sprite.pos[1];
            scene_sprite.position.x= sprite.pos[0];
            scene_sprite.scale.z= sprite.scale[2];
            scene_sprite.scale.y= sprite.scale[1];
            scene_sprite.scale.x= sprite.scale[0];
            scene_sprite.rotation.z= sprite.rotation[2];
            scene_sprite.rotation.y= sprite.rotation[1];
            scene_sprite.rotation.x= sprite.rotation[0];
            scene_sprite.receiveShadow = true;
            scene_sprite.castShadow = true;
            break;
        default:
            break;
    }
    if (!scene_sprite) {
        return;
    }
    scene.add(scene_sprite);
    all_scene_sprites[""+sprite.key] = scene_sprite;
}

function remove_sprite_from_scene_by_key(key) {
    var sprite = all_scene_sprites[""+key];
    delete all_scene_sprites[""+key];
    scene.remove(sprite)
}

function msg_push_as_world(msg) {
    if (msg.hasOwnProperty("session_id")) {
        session_id = msg.session_id;
    }
    if (msg.hasOwnProperty("hero_key")) {
        hero_key = msg.hero_key;
    }
    var world;
    if (worlds.length>0) {
        world = clone(worlds[worlds.length-1]);
    } else {
        world = {};
        world.sprites = {};
    }
    world.t = msg.t;
    for (var i=0; i<msg.new_sprites.length; i++) {
        var sprite = msg.new_sprites[i];
        world.sprites[""+sprite.key] = sprite;
        add_new_sprite_to_scene(sprite);
    }
    for (var i=0; i<msg.updated_sprites.length; i++) {
        var sprite = msg.updated_sprites[i];
        world.sprites[""+sprite.key] = sprite;
    }
    for (var i=0; i<msg.removed_sprite_keys.length; i++) {
        var key = msg.removed_sprite_keys[i];
        delete world.sprites[""+key];
        remove_sprite_from_scene_by_key(key);
    }
    if (!time_has_started) {
        console.log(msg);
        console.log(world);
    }
    worlds.push(world);
}

world_socket.onmessage = function (event) {
    if (event.data==="Hello") {
        console.log("world socket connected to server");
    } else {
        msg_push_as_world(JSON.parse(event.data));
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
    }, 40);
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
	renderer.render( scene, camera );



    stats.end();

    requestAnimationFrame( animate );

}

requestAnimationFrame( animate );





