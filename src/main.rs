mod actions;
mod structs;

fn main() {
    actions::create_object(
        "another",
        vec![(
            "name".to_string(),
            structs::DataType::Str("Andrei wads".to_string()),
        )],
        "example.sotdb",
    )
    .unwrap();
}
