use crate::structs::*;
use std::{fs::*, os::windows::prelude::FileExt, io::Write};

pub fn create_table(table_name: &str, data: Vec<&str>, path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let mut perm = file.metadata()?.permissions();
    perm.set_readonly(false);
    let mut tables = load_tables(path);
    Ok(())
}
fn load_tables(path: &str) -> Result<Vec<Table>, std::io::Error> {
    let mut tables = vec![];
    let mut table_names = vec![];
    let mut table_datas = vec![];
    read_to_string(path)?.lines().for_each(|line| {
        println!("{}", line);
        if line.contains("[table-name]:") {
            table_names.push(line.replace("[table-name]:", ""));
        }
        table_names.iter().for_each(|name| {
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

                table_datas.push((
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

                table_datas.push((
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

                table_datas.push((
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

                table_datas.push((
                    s[..var_end].to_string(),
                    DataType::Bool(s[var_end..].to_string().replace("=", "").parse().unwrap()),
                    name.to_owned(),
                ));
            }
        });
    });
    for name in table_names {
        let mut datas = vec![];
        for data in &table_datas {
            if data.2 == name {
                datas.push((data.0.to_owned(), data.1.to_owned()));
            }
        }
        tables.push(Table::new(name, datas))
    }
    println!("[Tables]:{:#?}", &tables);
    return Ok(tables);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        assert_eq!(
            create_table("example", vec!["name", "value"], "example.sotdb").unwrap(),
            ()
        );
    }
}
