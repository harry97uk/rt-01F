mod vector3;
mod colour;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;
mod material;
mod quad;
mod aabb;
mod bvh;
mod cylinder;
mod helper;

use std::collections::HashMap;

use camera::Camera;

use helper::{ extract_materials, extract_objects, get_nested_yaml_value };
use serde_yaml::Value;

use crate::hittable_list::HittableList;
use crate::ray::Ray;

fn main() {
    // World
    let mut world = HittableList::new();

    let yaml_content = std::fs
        ::read_to_string("/Users/harrygardiner/rt-01F/rt/config.yaml")
        .unwrap();
    let yaml_value: Value = serde_yaml::from_str(&yaml_content).unwrap();
    let mut materials = HashMap::new();

    // Extract materials configuration
    if let Some(materials_value) = yaml_value.get("materials") {
        materials = extract_materials(materials_value).unwrap();
    } else {
        eprintln!("'materials' key not found in the YAML file");
    }

    if let Some(object_values) = yaml_value.get("objects") {
        let objects = extract_objects(object_values, &materials).unwrap();

        for obj in objects {
            world.add(obj);
        }
    }

    // Camera
    let mut cam: Camera = match
        Camera::from_yaml_file("/Users/harrygardiner/rt-01F/rt/config.yaml")
    {
        Ok(camera) => {
            // Use the camera instance as needed
            //println!("{:?}", camera);
            camera
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Camera::new()
        }
    };

    let filename_value: Option<String> = get_nested_yaml_value(&yaml_value, "filename");

    let filename = match filename_value {
        Some(name) => name,
        None => "output_image".to_string(), // Provide a default filename if it's not present
    };
    cam.render(&world, filename)
}
