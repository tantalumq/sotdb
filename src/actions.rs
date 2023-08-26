use crate::structs::*;
use std::{
    fs::{self, *},
    io::{Error, ErrorKind, Write},
};

pub fn create_object(
    object_name: &str,
    datas: &mut Vec<(String, DataType)>,
    path: &str,
) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let mut perm = file.metadata()?.permissions();
    perm.set_readonly(false);
    let mut objects = load_objects(path)?;

    for object in objects.iter() {
        if object.get_name() == object_name {
            return Err(Error::new(ErrorKind::Other, "Name already created"));
        }
    }
    objects.push(Object::new(object_name.to_string(), datas.to_owned()));
    save_objects(objects, path.to_string())
}
pub fn get_object(object_name: &str, path: &str) -> Result<Object, std::io::Error> {
    let objects = load_objects(path)?;
    for object in objects {
        if object.get_name() == object_name {
            return Ok(object);
        }
    }
    return Err(Error::new(ErrorKind::Other, "Object not found"));
}
fn load_objects(path: &str) -> Result<Vec<Object>, std::io::Error> {
    let mut objects = vec![];
    let mut object_names = vec![];
    let mut object_datas = vec![];
    read_to_string(path)?.lines().for_each(|line| {
        if line.contains("[object-name]:") {
            object_names.push(line.replace("[object-name]:", ""));
        }
        object_names.iter().for_each(|name| {
            if line.contains(
                format!(
                    "[{}:{}]:",
                    name,
                    DataType::Str("_".to_string()).get_type_anotation()
                )
                .as_str(),
            ) {
                let s = line.replace(
                    format!(
                        "[{}:{}]:",
                        name,
                        DataType::Str("_".to_string()).get_type_anotation()
                    )
                    .as_str(),
                    "",
                );
                let var_end = s
                    .char_indices()
                    .find_map(|(idx, char)| if char == '=' { Some(idx) } else { None })
                    .unwrap();

                object_datas.push((
                    s[..var_end].to_string(),
                    DataType::Str(s[var_end..].to_string().replace("=", "")),
                    name.to_owned(),
                ));
            } else if line
                .contains(format!("[{}:{}]", name, DataType::Int(0).get_type_anotation()).as_str())
            {
                let s = line.replace(
                    format!("[{}:{}]:", name, DataType::Int(0).get_type_anotation()).as_str(),
                    "",
                );
                let var_end = s
                    .char_indices()
                    .find_map(|(idx, char)| if char == '=' { Some(idx) } else { None })
                    .unwrap();

                object_datas.push((
                    s[..var_end].to_string(),
                    DataType::Int(s[var_end..].to_string().replace("=", "").parse().unwrap()),
                    name.to_owned(),
                ));
            } else if line.contains(
                format!("[{}:{}]", name, DataType::Float(0.0).get_type_anotation()).as_str(),
            ) {
                let s = line.replace(
                    format!("[{}:{}]:", name, DataType::Float(0.0).get_type_anotation()).as_str(),
                    "",
                );
                let var_end = s
                    .char_indices()
                    .find_map(|(idx, char)| if char == '=' { Some(idx) } else { None })
                    .unwrap();

                object_datas.push((
                    s[..var_end].to_string(),
                    DataType::Float(s[var_end..].to_string().replace("=", "").parse().unwrap()),
                    name.to_owned(),
                ));
            }
            if line.contains(
                format!("[{}:{}]", name, DataType::Bool(false).get_type_anotation()).as_str(),
            ) {
                let s = line.replace(
                    format!("[{}:{}]:", name, DataType::Bool(false).get_type_anotation()).as_str(),
                    "",
                );
                let var_end = s
                    .char_indices()
                    .find_map(|(idx, char)| if char == '=' { Some(idx) } else { None })
                    .unwrap();

                object_datas.push((
                    s[..var_end].to_string(),
                    DataType::Bool(s[var_end..].to_string().replace("=", "").parse().unwrap()),
                    name.to_owned(),
                ));
            }
        });
    });
    for name in object_names {
        let mut datas = vec![];
        for data in &object_datas {
            if data.2 == name {
                datas.push((data.0.to_owned(), data.1.to_owned()));
            }
        }
        objects.push(Object::new(name, datas))
    }
    return Ok(objects);
}
fn save_objects(objects: Vec<Object>, path: String) -> Result<(), std::io::Error> {
    fs::remove_file(&path)?;
    let mut file = File::create(&path)?;
    for object in objects.iter() {
        file.write_all(format!("[object-name]:{}\n", object.get_name().trim_end()).as_bytes())?;
        for data in object.to_owned().get_data().iter() {
            file.write_all(
                format!(
                    "[{}:{}]:{}={}\n",
                    object.get_name().trim_end(),
                    data.1.get_type_anotation(),
                    data.0,
                    data.1.get_value()
                )
                .as_bytes(),
            )?;
        }
        file.write_all(format!("|=============================|\n").as_bytes())?;
    }
    Ok(())
}
