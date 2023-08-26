mod actions;
mod structs;

#[cfg(test)]
mod tests {
    use crate::{actions, structs::DataType};

    #[test]
    fn create_object() {
        assert_eq!((), actions::create_object(
            "test-create-object",
            &mut vec![
                ("test-str".to_string(), DataType::Str("test".to_string())),
                ("test-int".to_string(), DataType::Int(0)),
                ("test-float".to_string(), DataType::Float(0.0)),
                ("test-bool".to_string(), DataType::Bool(false)),
            ],
            "tests/test1.sotdb",
        ).unwrap());
    }
}
