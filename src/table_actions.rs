use std::fs::*;
use crate::structs::*;

pub fn create_table(table_name: &str, data: Vec<&str>, path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let mut perm = file.metadata()?.permissions();
    perm.set_readonly(false);
    read_to_string(path)?.lines().for_each(|line| {
        println!("{}", line);

    });
    Ok(())
}
fn load_tables(path: &str) -> Result<Vec<Table>, std::io::Error> {
    let mut tables = vec![];
    read_to_string(path)?.lines().for_each(|line| {
        println!("{}", line);
        
    });
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
