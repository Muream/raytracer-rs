pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            spheres: vec![]
        }
    }
}

pub struct Sphere {
    pub position: glm::Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self{
        Sphere {
            position: glm::Vec3::zeros(),
            radius: 1.0,
            material: Material::default(),
        }
    }
}

pub struct Material {
    pub albedo: glm::Vec4,
}

impl Default for Material {
    fn default() -> Self{
        Material {
            albedo: glm::vec4(1.0, 1.0, 1.0, 0.0)
        }
    }
}
