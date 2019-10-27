import * as Wasm from './wasm'
import Fps from './fps'

const canvas = {
    width: 480,
    height: 360
};

const getScene = () => {

    return {
        camera: {
            point: {
                x: 0,
                y: -5,
                z: -10,
            },
            vector: {
                x: 0,
                y: .25,
                z: 0
            },
            fov: 70
        },
        objects: [
            {
                type: 'Sphere',
                point: { x: -3, y: -1.0, z: 0 },
                color: { x: 200, y: 0, z: 0 },
                specular: 0.7,
                lambert: 0.2,
                ambient: 0.2,
                radius: 1.0
            },
            {
                type: 'Sphere',
                point: { x: 3, y: -1.0, z: 0 },
                color: { x: 0, y: 200, z: 0 },
                specular: 0.7,
                lambert: 0.2,
                ambient: 0.2,
                radius: 1.0
            },
            {
                type: 'Sphere',
                point: { x: 0, y: -1.5, z: 0 },
                color: { x: 0.0, y: 0, z: 200 },
                specular: 0.72,
                lambert: 0.2,
                ambient: 0.2,
                radius: 1.5
            },
            {
                type: 'Plane',
                point: { x: 0, y: 0, z: 0 },
                normal: { x: 0, y: -1, z: 0 },
                color: { x: 200, y: 200, z: 200 },
                specular: 0.0,
                lambert: 0.9,
                ambient: 0.2,
            },
        ],
        checker: [
            {
                x: 50,
                y: 0,
                z: 89
            },
            {
                x: 209,
                y: 209,
                z: 209
            }
        ],
        lights: [{
            x: 3,
            y: -5,
            z: -2
        }],
        distance_fog: {
            start_distance: 5,
            length: 120,
            color: {
                x: 28,
                y: 28,
                z: 28
            }
        }
    };
};

const putData = (data) => {
    const ctx = document.getElementById('canvas').getContext('2d');
    ctx.putImageData(new ImageData(new Uint8ClampedArray(data), canvas.width, canvas.height), 0, 0);
};

const renderWasm = (scene) => {

    const data = Wasm.render(canvas, scene);
    if (data) {         // may return undefined if wasm module not loaded
        putData(data);
    }
};

let inc = 0;
const fps = new Fps(250,  document.querySelector('.fps'));

const render = () => {

    fps.tick();

    const scene = getScene();

    scene.objects[0].point.x = Math.sin(inc) * 3.0;
    scene.objects[0].point.z = Math.cos(inc) * 3.0;

    scene.objects[1].point.x = Math.sin(inc) * -3.0;
    scene.objects[1].point.z = Math.cos(inc) * -3.0;

    inc += parseFloat(0.06);

    renderWasm(scene);

    requestAnimationFrame(render);
};

requestAnimationFrame(render);
