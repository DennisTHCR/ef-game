use noise::{NoiseFn, Perlin};

use crate::structs::world::Material;

pub fn get_material(x: i32, y: i32) -> Material {
    if !is_stone(x, y) {
        return Material::GRASS
    };
    let mut material: Material = Material::STONE;
    if is_emerald(x, y) {
        material = Material::EMERALD;
    }
    if is_diamond(x,y) {
        material = Material::DIAMOND;
    }
    if is_iron(x, y) {
        material = Material::IRON;
    }
    if is_coal(x, y) {
        material = Material::COAL;
    }
    material
}

fn get_height_value(x: i32, y: i32) -> f64 {
    Perlin::new(0).get([x as f64 / 16., y as f64 / 16.])
}

fn is_stone(x: i32, y: i32) -> bool {
    // The bigger the division of the x/y coordinates, the higher the resolution, the bigger the patches
    let height_value = get_height_value(x, y);
    height_value >= 0.5
}

fn is_emerald(x: i32, y: i32) -> bool {
    let noise_1 = Perlin::new(45).get([x as f64 / 16., y as f64 / 32.]);
    let noise_2 = Perlin::new(99).get([x as f64 / 32., y as f64 / 16.]);
    let noise_3 = Perlin::new(10).get([x as f64 / 8., y as f64 / 4.]);
    let noise_4 = Perlin::new(89).get([x as f64 / 64., y as f64 / 12.]);
    noise_1 > 0.6 && noise_2 > 0.6 && noise_3 > 0.6 && noise_4 > 0.6
}

fn is_diamond(x: i32, y: i32) -> bool {
    let noise_1 = Perlin::new(99).get([x as f64 / 32., y as f64 / 19.]);
    let noise_2 = Perlin::new(45).get([x as f64 / 16., y as f64 / 13.]);
    let noise_3 = Perlin::new(32).get([x as f64 / 48., y as f64 / 12.]);
    noise_1 > 0.5 && noise_2 > 0.5 && noise_3 > 0.5
}

fn is_iron(x: i32, y: i32) -> bool {
    let noise_1 = Perlin::new(99).get([x as f64 / 8., y as f64 / 16.]);
    let noise_2 = Perlin::new(40).get([x as f64 / 32., y as f64 / 16.]);
    noise_1 > 0.5 && noise_2 > 0.5
}

fn is_coal(x: i32, y: i32) -> bool {
    let noise_1 = Perlin::new(5).get([x as f64 / 16., y as f64 / 48.]);
    let noise_2 = Perlin::new(200).get([x as f64 / 32., y as f64 / 16.]);
    noise_1 > 0.4 && noise_2 > 0.4
}