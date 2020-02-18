const {actions} = require('./slice');
const THREE = require('three');
// We don't support ES6 import statements, so have to bring THREE into window
// scope and require the GLTFLoader separately.
window.THREE = THREE;
require('three/examples/js/loaders/GLTFLoader.js');

// Reuse the same renderer
const renderer = new THREE.WebGLRenderer();
let scene = null;
let position = new THREE.Vector3();
let rotation = new THREE.Quaternion();
let scale = new THREE.Vector3();

const loadGLTF = path => dispatch => {
  const loader = new THREE.GLTFLoader();
  loader.load(path,
    (gltf) => {
      // Cache the scene outside of redux
      scene = gltf.scene;

      // load the camera info first before loading the model
      const cameras = gltf.cameras.map(camera => camera.name);
      if (cameras.length > 0) {
        dispatch(loadCamera(cameras[0]));
      }

      dispatch(actions.loadModel({path, cameras}));
    },
    null, // onProgress
    err => console.error(err),
  );
};

const loadCamera = cameraName => dispatch => {
  let cameraObj = scene.getObjectByName(cameraName);
  // Render the scene with the camera so that the camera properties are
  // updated with respect to the world.
  renderer.render(scene, cameraObj);
  cameraObj.matrixWorld.decompose(position, rotation, scale);

  dispatch(actions.setCamera({
    name: cameraName,
    position: position.toArray(),
    rotation: rotation.toArray(),
    scale: scale.toArray(),
    // TODO: handle non-perspective cameras
    aspect_ratio: cameraObj.aspect,
    near_z: cameraObj.near,
    far_z: cameraObj.far,
    fov: cameraObj.fov,
  }));
};

module.exports = {
  loadGLTF,
  loadCamera,
};