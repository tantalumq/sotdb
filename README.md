# Storage of tantalum

Sot is a database written in Rust.

## Installation

Use cargo to install the library.
```
cargo add sotdb
```
## Usage

```rust
use sotdb::{actions::*, structs::*};

fn main() -> Result<(), std::io::Error> {
    // Create object (requires name, path to *.sotdb file, and vec of data: (var name, datatype(data)))
    let path = "*.sotdb"
    create_object(
        "name",
        &mut vec![
            // text, intnum, floatnum, boolean - name of var`s
            // Str("Text".to_string()), Int(0), Float(0.0), Bool(false) - datatype`s with with their data
            ("text".to_string(), DataType::Str("test".to_string())),
            ("intnum".to_string(), DataType::Int(0)),
            ("floatnum".to_string(), DataType::Float(0.0)),
            ("boolean".to_string(), DataType::Bool(false)),
        ],
        path,
    )?;
    // Get one object (requires name and path to *.sotdb file)
    let object = get_object("name", path)?;
    // Get all objects from file (reguires path to *.sotdb file)
    let _all_objects = get_all_objects(path)?;
    // Add data to object (requires name, path  to *.sotdb file, and vec of data: (String, DataType))
    add_data_to_object(
        object.get_name(),
        path,
        vec![("boolean".to_string(), DataType::Bool(true))],
    )?;
    // Remove data from object (requires name, path to *.sotdb file, and vec of data: (var name, datatype(data)))
    remove_data_from_object(object.get_name(), path, object.get_data())?;
    // Delete object (requires name and path to *.sotdb file)
    delete_object(object.get_name(), path)?;
    Ok(())
    
    // Program writes to file:
    // [object-name]:name
    // [name:<str>]:text=test
    // [name:<int>]:intnum=0
    // [name:<float>]:floatnum=0
    // [name:<bool>]:boolean=false
    // |=============================|
    // And delete this all! :)

}
```
## License

[MIT](https://choosealicense.com/licenses/mit/)
