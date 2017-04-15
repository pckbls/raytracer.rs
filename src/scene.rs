use camera::Camera;
use model::Model;
use lighting::LightSource;

#[allow(dead_code)]
pub struct Scene {
    pub models: Vec<Model>,
    pub light_sources: Vec<LightSource>,
    pub camera: Camera
}

