//! Module to handle materials

use uuid::Uuid;
use crate::vulkan_wrapper::texture::Texture;

pub struct Material {
    pub id: Uuid,
    pub texture: Texture,
}

impl Material {
    pub fn new(texture: Texture) -> Material {
        Material {
        id: Uuid::new_v4(),
        texture: texture,
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
    fn test_material_constructor() {
        let mat = Material::new(
            Texture::GreyWall,
        );
        assert_eq!(mat.id.is_nil(), false);
        assert_eq!(mat.texture == Texture::GreyWall, true);
    }
}

