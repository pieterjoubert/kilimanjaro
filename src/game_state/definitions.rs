//! Module to manage game definitions
use cgmath::prelude::*;
use cgmath::{Vector3, Vector4};
use uuid::Uuid;

use crate::asset_manager::materials::Material;
use crate::vulkan_wrapper::texture::Texture;
use crate::vulkan_wrapper::shader::LightShader;

pub struct Level {
    pub id: Uuid,
    pub file_name: String,
    pub rooms: Vec<Room>,
    pub doors: Vec<Door>,
}

impl Level {
    
}

pub struct Door {
    pub id: Uuid,
    pub first_room: Room,
    pub second_room: Room,
    pub width: f64,
    pub inner_texture: Material,
}

impl Door {
    
}

pub struct Room {
    pub id: Uuid,
    pub corner_pos: Vector3<f64>,
    pub width: f64,  //x-axis
    pub depth: f64,  //z-axis
    pub height: f64, //y-axis
    pub textures: Vec<Material>,
    pub lights: Vec<Light>,
}

impl Room {
    pub fn new(
        corner_pos: Vector3<f64>,
        width: f64,
        depth: f64,
        height: f64,
        textures: Vec<Material>,
        lights: Vec<Light>,
    ) -> Room {
        Room {
            id: Uuid::new_v4(),
            corner_pos,
            width,
            depth,
            height,
            textures,
            lights,
        }
    }
}


pub struct Light {
    pub id: Uuid,
    pub pos: Vector3<f64>,
    pub rot: Vector4<f64>,
    pub light_shader: LightShader,
}

impl Light {
    pub fn new(
        pos: Vector3<f64>,
        rot: Vector4<f64>,
        light_shader: LightShader,
    ) -> Light {
        Light {
            id: Uuid::new_v4(),
            pos,
            rot,
            light_shader,
        }
    }
}
// ------------------- TESTS -------------------

// Function to test module linking

pub fn mod_found() -> bool {
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_room_constructor() {
        let room = Room::new(
            Vector3::new(0.0, 0.0, 0.0),
            10.0,
            10.0,
            2.0,
            vec![Material::new(Texture::GreyWall)],
            vec![Light::new(Texture::GreyWall)],
        );
        assert_eq!(room.id.is_nil(), false);
        assert_eq!(room.corner_pos.magnitude() == 0.0, true);
        assert_eq!(room.width == 10.0, true);
        assert_eq!(room.depth == 10.0, true);
        assert_eq!(room.height == 2.0, true);
        assert_eq!(room.textures[0].texture == Texture::GreyWall, true);
        assert_eq!(room.lights[0] == Texture::GreyWall, true);
    }
}
