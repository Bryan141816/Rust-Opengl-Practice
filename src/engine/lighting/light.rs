use glam::Vec3;

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,

    pub cut_off: f32,
    pub outer_cut_off: f32,
}
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}
