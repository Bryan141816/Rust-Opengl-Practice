use crate::engine::{
    scene::components::Light,
    backend::opengl::OpenGLShader,

};
use glow::HasContext;

pub struct LightUniforms;

impl LightUniforms {
    pub unsafe fn upload(
        gl: &glow::Context,
        shader: &OpenGLShader,
        lights: &[Light],
    ) {
        let mut directional_count = 0;
        let mut point_count = 0;
        let mut spot_count = 0;
        unsafe {
            for light in lights {
                match light {
                    Light::Directional(light) => {
                        let prefix = format!("directionalLights[{}]", directional_count);

                        shader.set_vec3(gl, &format!("{}.direction", prefix), light.direction);
                        shader.set_vec3(gl, &format!("{}.color", prefix), light.color);
                        shader.set_float(gl, &format!("{}.intensity", prefix), light.intensity);

                        directional_count += 1;
                    }

                    Light::Point(light) => {
                        let prefix = format!("pointLights[{}]", point_count);

                        shader.set_vec3(gl, &format!("{}.position", prefix), light.position);
                        shader.set_vec3(gl, &format!("{}.color", prefix), light.color);
                        shader.set_float(gl, &format!("{}.intensity", prefix), light.intensity);
                        shader.set_float(gl, &format!("{}.constant", prefix), light.constant);
                        shader.set_float(gl, &format!("{}.linear", prefix), light.linear);
                        shader.set_float(gl, &format!("{}.quadratic", prefix), light.quadratic);

                        point_count += 1;
                    }

                    Light::Spot(light) => {
                        let prefix = format!("spotLights[{}]", spot_count);

                        shader.set_vec3(gl, &format!("{}.position", prefix), light.position);
                        shader.set_vec3(gl, &format!("{}.direction", prefix), light.direction);
                        shader.set_vec3(gl, &format!("{}.color", prefix), light.color);
                        shader.set_float(gl, &format!("{}.intensity", prefix), light.intensity);
                        shader.set_float(gl, &format!("{}.cutOff", prefix), light.cut_off);
                        shader.set_float(gl, &format!("{}.outerCutOff", prefix), light.outer_cut_off);

                        spot_count += 1;
                    }
                }
            }

            shader.set_int(gl, "directionalLightCount", directional_count);
            shader.set_int(gl, "pointLightCount", point_count);
            shader.set_int(gl, "spotLightCount", spot_count);
        }
    }
}