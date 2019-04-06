use std::rc::Rc;

use vek::Vec3;

use crate::material::Material;

#[derive(Debug)]
pub struct Mesh {
    indices: Vec<u32>,
    /// The position of each vertex of the model
    positions: Vec<Vec3<f32>>,
    /// The normal of each vertex of the model
    normals: Vec<Vec3<f32>>,
    /// The material associated with this mesh (if any)
    material: Rc<Material>,
}

impl Mesh {
    pub fn from_obj(mesh: tobj::Mesh, materials: &[Rc<Material>]) -> Self {
        Self {
            indices: mesh.indices,
            positions: mesh.positions.chunks(3).map(|sl| Vec3::from_slice(sl)).collect(),
            normals: mesh.normals.chunks(3).map(|sl| Vec3::from_slice(sl)).collect(),
            material: mesh.material_id.map(|id| materials[id].clone()).unwrap_or_default(),
        }
    }

    pub fn from_gltf(
        buffers: &Vec<gltf::buffer::Data>,
        primitive: &gltf::Primitive,
        materials: &[Rc<Material>],
    ) -> Self {

        // We're only dealing with triangle meshes
        assert_eq!(gltf::mesh::Mode::Triangles, primitive.mode());

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions = reader.read_positions().expect("Failed to read glTF positions").collect::<Vec<_>>();
        let normals = reader.read_normals().expect("Failed to read glTF normals").collect::<Vec<_>>();
        let indices = reader
            .read_indices()
            .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
            .expect("Failed to read glTF indices");

        // Not handling optional normals yet
        assert_eq!(
            positions.len(),
            normals.len(),
            "Position vector and normals vector have different lengths"
        );

        Self {
            indices: indices,
            positions: positions.iter().map(|data| Vec3::new(data[0], data[1], data[2])).collect(),
            normals: normals.iter().map(|data| Vec3::new(data[0], data[1], data[2])).collect(),
            material: primitive.material().index().map(|id| materials[id].clone()).unwrap_or_default(),
        }
    }

    /// Returns the indices of this mesh
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Returns the position for the given index
    pub fn position(&self, index: usize) -> Vec3<f32> {
        self.positions[index]
    }

    /// Returns the normal for the given index
    pub fn normal(&self, index: usize) -> Vec3<f32> {
        self.normals[index]
    }

    /// Returns the material associated with this mesh
    pub fn material(&self) -> &Material {
        &self.material
    }
}
