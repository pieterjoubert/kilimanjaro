//! Module to define a kilimanjaro texture

use vulkano::format::*;
use vulkano::image::StorageImage;

#[derive(PartialEq)]
pub enum Texture {
    Grass,
    Brick,
    Metal,
    Glass,
    Stone,
    GreyWall,
    Wood,
    Sand,
    Water,
    Door,
    Default,
    //pub image: ,
}
