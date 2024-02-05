use std::{ collections::HashMap, rc::Rc };

use serde::de::DeserializeOwned;
use serde_yaml::Value;

use crate::{
    material::{ Material, Lambertian, Metal },
    colour::Colour,
    hittable::Hittable,
    sphere::Sphere,
    cylinder::Cylinder,
    vector3::Vector3,
    quad::{ Plane, cuboid },
};

// Helper function to get values from nested YAML structures
// pub fn get_yaml_value<T>(yaml_value: &Value, key: &str) -> Result<T, Box<dyn std::error::Error>>
//     where T: serde::de::DeserializeOwned
// {
//     match yaml_value {
//         Value::Mapping(mapping) => {
//             if let Some(value) = mapping.get(&serde_yaml::Value::String(key.to_string())) {
//                 let result: T = serde_yaml::from_value(value.clone())?;
//                 Ok(result)
//             } else {
//                 Err(format!("Key '{}' not found", key).into())
//             }
//         }
//         _ => Err("Invalid YAML structure".into()),
//     }
// }

pub fn get_nested_yaml_value<'a, T>(yaml_value: &'a Value, key: &str) -> Option<T>
    where T: DeserializeOwned
{
    if let Value::Mapping(mapping) = yaml_value {
        if let Some(value) = mapping.get(&serde_yaml::Value::String(key.to_string())) {
            if let Ok(deserialized_value) = serde_yaml::from_value(value.clone()) {
                return Some(deserialized_value);
            }
        }
    }
    None
}

pub fn extract_materials(
    materials_value: &Value
) -> Result<HashMap<String, Rc<dyn Material>>, Box<dyn std::error::Error>> {
    let mut materials = HashMap::new();

    if let Value::Sequence(materials_seq) = materials_value {
        for material_entry in materials_seq {
            if let Value::Mapping(material_mapping) = material_entry {
                if let Some((material_name, material_properties)) = material_mapping.iter().next() {
                    let material_type_value = get_nested_yaml_value(
                        material_properties,
                        "type"
                    ).unwrap_or("Lambertian".to_string());

                    let material_colour_values = get_nested_yaml_value(
                        material_properties,
                        "colour"
                    ).unwrap_or([1.0, 1.0, 1.0]);

                    let material_colour = Colour::new(
                        material_colour_values[0],
                        material_colour_values[1],
                        material_colour_values[2]
                    );

                    let material: Rc<dyn Material> = match material_type_value.as_str() {
                        "Lambertian" => Rc::new(Lambertian::new(material_colour)),
                        "Metal" => Rc::new(Metal::new(material_colour)),
                        _ => Rc::new(Lambertian::new(material_colour)),
                    };

                    let material_name_str = material_name
                        .as_str()
                        .expect("material name is not a string");

                    materials.insert(material_name_str.to_string(), material);
                } else {
                    eprintln!("Material entry is missing 'type' or 'colour' key");
                }
            } else {
                eprintln!("Invalid material entry in the YAML file");
            }
        }
    } else {
        eprintln!("'materials' key should be a sequence in the YAML file");
    }

    Ok(materials)
}

pub fn extract_objects(
    objects_value: &Value,
    materials: &HashMap<String, Rc<dyn Material>>
) -> Result<Vec<Rc<dyn Hittable>>, Box<dyn std::error::Error>> {
    let mut objects: Vec<Rc<dyn Hittable>> = Vec::new();

    if let Value::Sequence(objects_seq) = objects_value {
        for object_config in objects_seq {
            let object_type: String = get_nested_yaml_value(object_config, "type").expect(
                "no object defined"
            );

            match object_type.as_str() {
                "Plane" => {
                    let q_value = get_nested_yaml_value(object_config, "q").unwrap_or([
                        -6.0, -0.5, -6.0,
                    ]);
                    let u_value = get_nested_yaml_value(object_config, "u").unwrap_or([
                        12.0, 0.0, 0.0,
                    ]);
                    let v_value = get_nested_yaml_value(object_config, "v").unwrap_or([
                        0.0, 0.0, 12.0,
                    ]);

                    let q = Vector3::new(q_value[0], q_value[1], q_value[2]);
                    let u = Vector3::new(u_value[0], u_value[1], u_value[2]);
                    let v = Vector3::new(v_value[0], v_value[1], v_value[2]);

                    let material_name = get_nested_yaml_value(object_config, "mat").unwrap_or(
                        "".to_string()
                    );

                    let plane = Plane::new(
                        q,
                        u,
                        v,
                        materials.get(&material_name).expect("could not find material name").clone()
                    );

                    objects.push(Rc::new(plane));
                }
                "Sphere" => {
                    let centre_value = get_nested_yaml_value(object_config, "centre").unwrap_or([
                        0.0, 0.0, 0.0,
                    ]);
                    let centre = Vector3::new(centre_value[0], centre_value[1], centre_value[2]);

                    let radius = get_nested_yaml_value(object_config, "radius").unwrap_or(1.0);

                    let material_name = get_nested_yaml_value(object_config, "mat").unwrap_or(
                        "".to_string()
                    );

                    let sphere = Sphere::new(
                        centre,
                        radius,
                        materials.get(&material_name).expect("could not find material name").clone()
                    );
                    objects.push(Rc::new(sphere));
                }
                "Cuboid" => {
                    let a_value = get_nested_yaml_value(object_config, "a").unwrap_or([
                        -1.0, -0.5, -1.0,
                    ]);
                    let b_value = get_nested_yaml_value(object_config, "b").unwrap_or([
                        1.0, 0.5, 1.0,
                    ]);

                    let a = Vector3::new(a_value[0], a_value[1], a_value[2]);
                    let b = Vector3::new(b_value[0], b_value[1], b_value[2]);

                    let material_name = get_nested_yaml_value(object_config, "mat").unwrap_or(
                        "".to_string()
                    );

                    let cuboid = cuboid(
                        a,
                        b,
                        materials.get(&material_name).expect("could not find material name").clone()
                    );

                    objects.push(cuboid);
                }
                "Cylinder" => {
                    let centre_value = get_nested_yaml_value(object_config, "centre").unwrap_or([
                        0.0, 0.5, 0.0,
                    ]);
                    let centre = Vector3::new(centre_value[0], centre_value[1], centre_value[2]);

                    let radius = get_nested_yaml_value(object_config, "radius").unwrap_or(0.6);

                    let height = get_nested_yaml_value(object_config, "height").unwrap_or(1.0);

                    let material_name = get_nested_yaml_value(object_config, "mat").unwrap_or(
                        "".to_string()
                    );

                    let cylinder = Cylinder::new(
                        centre,
                        radius,
                        height,
                        materials.get(&material_name).expect("could not find material name").clone()
                    );
                    objects.push(Rc::new(cylinder));
                }
                // Add more object types as needed
                _ => {
                    // Handle unknown object types or log an error
                    eprintln!("Unknown object type: {}", object_type);
                }
            }
        }
    }

    Ok(objects)
}
