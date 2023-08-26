# Storage of tantalum

Sot is a database written in Rust.

## Installation

Use the cargo cargo add sotdb.
```
cargo add sotdb
```
## Usage

```rust
use sotdb::{actions::*, structs::*};

fn main() {
    // Create object using name, vector of pair`s (var name, datatype(data))
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
        "path to file (*.sotdb)"
    );
    // Get one object using his name  and path to *.sotdb file
    let object = get_object("name", "path to file (*.sotdb)");
    // Get all objects from *.sotdb file
    let all_objects = get_all_objects("path to file (*.sotdb)");
    // Delete object using his name and path to *.sotdb file
    delete_object("name", "path to file (*.sotdb");
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
