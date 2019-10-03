//! Module to manage shaders
use cgmath::Vector4;
use uuid::Uuid;

pub struct ShaderInfo {

}

pub struct Shader {
    pub id: Uuid,
    pub input: Option<ShaderInfo>,
    pub code: String,
    pub output: Option<ShaderInfo>,
}

impl Shader {
    pub fn new(code: String) -> Shader {
        Shader {
            id: Uuid::new_v4(),
            input: None,
            code: code,
            output: None,
        }
    }

    pub fn with_input(&mut self, input: Option<ShaderInfo>) -> &mut Shader {
        self.input = input;
        self
    }

    pub fn with_output(&mut self, output: Option<ShaderInfo>) -> &mut Shader {
        self.output = output;
        self
    }

    pub fn build(self) -> Shader {
        Shader {
            id: self.id,
            input: self.input,
            code: self.code,
            output: self.output,
        }
    }
}

// -------------- LIGHT STUFF ----------------

pub enum LightType {
    Spot,
    Ambient,
    Directional,
}

pub struct LightShader {
    pub id: Uuid,
    pub light_type: LightType,
    pub color: Vector4<f64>,
    pub shader: Shader,
}

/// Function to test module linking

pub fn mod_found() -> bool {
    return true;
}
