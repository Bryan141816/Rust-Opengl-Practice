use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub color: Vec3,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub color: Vec3,
    pub intensity: f32,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    pub color: Vec3,
    pub intensity: f32,
    pub cut_off: f32,
    pub outer_cut_off: f32,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}
#[allow(dead_code)]
impl Light {
    pub fn directional(color: Vec3, intensity: f32) -> Self {
        Self::Directional(DirectionalLight {
            color,
            intensity,
        })
    }

    pub fn point(color: Vec3, intensity: f32) -> Self {
        Self::Point(PointLight {
            color,
            intensity,
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        })
    }

    pub fn spot(
        color: Vec3,
        intensity: f32,
        cut_off: f32,
        outer_cut_off: f32,
    ) -> Self {
        Self::Spot(SpotLight {
            color,
            intensity,
            cut_off: cut_off.to_radians().cos(),
            outer_cut_off: outer_cut_off.to_radians().cos(),
        })
    }
}