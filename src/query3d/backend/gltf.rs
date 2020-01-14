use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;

use crate::scene::{Scene, Traverse, Mesh, Material, CameraType};
use crate::renderer::{Display, ShaderGeometry, Camera, DirectionalLight};
use crate::query3d::{GeometryQuery, GeometryFilter, CameraQuery, LightQuery};

use super::{QueryBackend, QueryError};

/// Represents a single glTF file
#[derive(Debug)]
pub struct GltfFile {
    default_scene: usize,
    scenes: Vec<Arc<Scene>>,
    /// Cache the geometry of the entire scene, referenced by scene index
    scene_shader_geometry: HashMap<usize, Arc<Vec<Arc<ShaderGeometry>>>>,
}

impl GltfFile {
    /// Opens a glTF file
    pub fn open(path: &Path) -> Result<Self, gltf::Error> {
        let (document, buffers, _images) = gltf::import(path)?;

        let materials: Vec<_> = document.materials()
            .map(|mat| Arc::new(Material::from(mat)))
            .collect();
        let meshes: Vec<_> = document.meshes()
            .map(|mesh| Arc::new(Mesh::from_gltf(mesh, &materials, &buffers)))
            .collect();

        let cameras: Vec<_> = document.cameras()
            .map(|cam| Arc::new(CameraType::from(cam)))
            .collect();

        //TODO: Lights

        let scenes: Vec<_> = document.scenes()
            .map(|scene| Arc::new(Scene::from_gltf(scene, &meshes, &cameras)))
            .collect();
        assert!(!scenes.is_empty(), "glTF file must have at least one scene");

        // Get the default scene, or just use the first scene if no default is provided
        let default_scene = document.default_scene().map(|scene| scene.index()).unwrap_or(0);

        Ok(Self {
            default_scene,
            scenes,
            scene_shader_geometry: HashMap::new(),
        })
    }
}

impl QueryBackend for GltfFile {
    fn query_geometry(&mut self, query: &GeometryQuery, display: &Display) -> Result<Arc<Vec<Arc<ShaderGeometry>>>, QueryError> {
        let GeometryQuery {models, animation} = query;

        //TODO: Restructure the code in this file to add animation support

        use GeometryFilter::*;
        let scene_index = match models {
            Scene {name: None} => self.default_scene,
            // This assumes that scene names are unique. If they are not unique, we might need to
            // search for all matching scenes and produce an error if there is more than one result
            Scene {name: Some(name)} => self.scenes.iter()
                .position(|scene| scene.name.as_ref() == Some(name))
                .ok_or_else(|| QueryError::UnknownScene {name: name.clone()})?,
        };

        match self.scene_shader_geometry.get(&scene_index) {
            Some(scene_geo) => Ok(scene_geo.clone()),
            None => {
                let scene = &self.scenes[scene_index];

                let mut scene_geo = Vec::new();
                for (parent_trans, node) in scene.roots.iter().flat_map(|root| root.traverse()) {
                    let model_transform = parent_trans * node.transform;

                    if let Some(mesh) = node.mesh() {
                        for geo in &mesh.geometry {
                            let geo = ShaderGeometry::new(display, geo, model_transform)?;
                            scene_geo.push(Arc::new(geo));
                        }
                    }

                }

                let scene_geo = Arc::new(scene_geo);
                self.scene_shader_geometry.insert(scene_index, scene_geo.clone());
                Ok(scene_geo)
            },
        }
    }

    fn query_camera(&mut self, query: &CameraQuery) -> Result<Arc<Camera>, QueryError> {
        unimplemented!()
    }

    fn query_lights(&mut self, query: &LightQuery) -> Result<Arc<Vec<Arc<DirectionalLight>>>, QueryError> {
        unimplemented!()
    }
}