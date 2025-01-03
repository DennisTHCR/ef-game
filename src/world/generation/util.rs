use noise::{NoiseFn, Perlin};

use crate::structs::world::Material;

pub fn get_material(x: i32, y: i32) -> Material {
    if !is_stone(x, y) {
        if is_spawner(x, y) {
            return Material::Spawner;
        }
        return Material::Grass;
    };
    let mut material: Material = Material::Stone;
    if is_emerald(x, y) {
        material = Material::Emerald;
    }
    if is_diamond(x, y) {
        material = Material::Diamond;
    }
    if is_iron(x, y) {
        material = Material::Iron;
    }
    if is_coal(x, y) {
        material = Material::Coal;
    }
    material
}

fn get_height_value(x: i32, y: i32) -> f64 {
    Perlin::new(5).get([x as f64 / 16., y as f64 / 16.])
        + Perlin::new(16).get([x as f64 / 32., y as f64 / 32.])
}

fn is_stone(x: i32, y: i32) -> bool {
    // The bigger the division of the x/y coordinates, the higher the resolution, the bigger the patches
    let height_value = get_height_value(x, y);
    height_value >= 0.6
}

fn is_spawner(x: i32, y: i32) -> bool {
    Perlin::new(5).get([x as f64 / 2., y as f64 / 2.]) > 0.99
        && Perlin::new(200).get([x as f64 / 48., y as f64 / 8.]) > 0.8
}

fn is_emerald(x: i32, y: i32) -> bool {
    let height_value = get_height_value(x, y);
    let noise_1 = Perlin::new(45).get([x as f64 / 16., y as f64 / 32.]) * height_value;
    let noise_2 = Perlin::new(99).get([x as f64 / 32., y as f64 / 16.]) * height_value;
    let noise_3 = Perlin::new(10).get([x as f64 / 8., y as f64 / 4.]) * height_value;
    let noise_4 = Perlin::new(89).get([x as f64 / 64., y as f64 / 12.]) * height_value;
    noise_1 > 0.6 && noise_2 > 0.6 && noise_3 > 0.6 && noise_4 > 0.6
}

fn is_diamond(x: i32, y: i32) -> bool {
    let height_value = get_height_value(x, y);
    let noise_1 = Perlin::new(99).get([x as f64 / 32., y as f64 / 19.]) * height_value;
    let noise_2 = Perlin::new(45).get([x as f64 / 16., y as f64 / 13.]) * height_value;
    let noise_3 = Perlin::new(32).get([x as f64 / 48., y as f64 / 12.]) * height_value;
    noise_1 > 0.5 && noise_2 > 0.5 && noise_3 > 0.5
}

fn is_iron(x: i32, y: i32) -> bool {
    let height_value = get_height_value(x, y);
    let noise_1 = Perlin::new(99).get([x as f64 / 8., y as f64 / 16.]) * height_value;
    let noise_2 = Perlin::new(40).get([x as f64 / 32., y as f64 / 16.]) * height_value;
    noise_1 > 0.5 && noise_2 > 0.5
}

fn is_coal(x: i32, y: i32) -> bool {
    let height_value = get_height_value(x, y);
    let noise_1 = Perlin::new(5).get([x as f64 / 16., y as f64 / 48.]) * height_value;
    let noise_2 = Perlin::new(200).get([x as f64 / 48., y as f64 / 8.]) * height_value;
    noise_1 > 0.4 && noise_2 > 0.4
}
