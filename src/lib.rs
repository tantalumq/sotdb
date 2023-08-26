pub mod actions;
pub mod structs;
#[cfg(test)]
mod succes_tests {

    use crate::{
        actions,
        structs::{DataType, Object},
    };

    #[test]
    fn create_object() {
        assert_eq!(
            (),
            actions::create_object(
                "test-create-object",
                &mut vec![
                    ("test-str".to_string(), DataType::Str("test".to_string())),
                    ("test-int".to_string(), DataType::Int(0)),
                    ("test-float".to_string(), DataType::Float(0.0)),
                    ("test-bool".to_string(), DataType::Bool(false)),
                ],
                "tests/succes/test1.sotdb",
            )
            .unwrap()
        );
    }

    #[test]
    fn get_object() {
        actions::create_object(
            "test-get-object",
            &mut vec![
                ("test-str".to_string(), DataType::Str("test".to_string())),
                ("test-int".to_string(), DataType::Int(0)),
                ("test-float".to_string(), DataType::Float(0.0)),
                ("test-bool".to_string(), DataType::Bool(false)),
            ],
            "tests/succes/test2.sotdb",
        )
        .unwrap();
        assert_eq!(
            Object::new(
                "test-get-object".to_string(),
                vec![
                    ("test-str".to_string(), DataType::Str("test".to_string())),
                    ("test-int".to_string(), DataType::Int(0)),
                    ("test-float".to_string(), DataType::Float(0.0)),
                    ("test-bool".to_string(), DataType::Bool(false)),
                ],
            ),
            actions::get_object("test-get-object", "tests/succes/test2.sotdb",).unwrap()
        );
    }
    #[test]
    fn delete_object() {
        actions::create_object(
            "test-delete-object",
            &mut vec![
                ("test-str".to_string(), DataType::Str("test".to_string())),
                ("test-int".to_string(), DataType::Int(0)),
                ("test-float".to_string(), DataType::Float(0.0)),
                ("test-bool".to_string(), DataType::Bool(false)),
            ],
            "tests/succes/test3.sotdb",
        )
        .unwrap();
        assert_eq!(
            (),
            actions::delete_object("test-delete-object", "tests/succes/test3.sotdb",).unwrap()
        );
    }
}
#[cfg(test)]
mod failures_tests {
    use crate::{actions, structs::DataType};
    #[test]
    fn object_name_already_in_use() {
        actions::create_object(
            "test-error-create-object",
            &mut vec![
                ("test-str".to_string(), DataType::Str("test".to_string())),
                ("test-int".to_string(), DataType::Int(0)),
                ("test-float".to_string(), DataType::Float(0.0)),
                ("test-bool".to_string(), DataType::Bool(false)),
            ],
            "tests/failures/test1.sotdb",
        )
        .unwrap();
        assert_eq!(
            std::io::Error::new(std::io::ErrorKind::Other, "Name already in use").to_string(),
            actions::create_object(
                "test-error-create-object",
                &mut vec![
                    ("test-str".to_string(), DataType::Str("test".to_string())),
                    ("test-int".to_string(), DataType::Int(0)),
                    ("test-float".to_string(), DataType::Float(0.0)),
                    ("test-bool".to_string(), DataType::Bool(false)),
                ],
                "tests/failures/test1.sotdb",
            )
            .unwrap_err()
            .to_string()
        )
    }
    #[test]
    fn object_not_found() {
        assert_eq!(
            std::io::Error::new(std::io::ErrorKind::Other, "Object not found").to_string(),
            actions::get_object("test-error-not-found-object", "tests/failures/test2.sotdb",)
                .unwrap_err()
                .to_string()
        );
        assert_eq!(
            std::io::Error::new(std::io::ErrorKind::Other, "Object not found").to_string(),
            actions::delete_object("test-error-not-found-object", "tests/failures/test2.sotdb",)
                .unwrap_err()
                .to_string()
        )
    }
    #[test]
    fn invalid_file_extension() {
        assert_eq!(
            std::io::Error::new(std::io::ErrorKind::Other, "Invalid file extension").to_string(),
            actions::create_object(
                "test-error-invalid-file-extension",
                &mut vec![
                    ("test-str".to_string(), DataType::Str("test".to_string())),
                    ("test-int".to_string(), DataType::Int(0)),
                    ("test-float".to_string(), DataType::Float(0.0)),
                    ("test-bool".to_string(), DataType::Bool(false)),
                ],
                "tests/failures/test3.txt",
            )
            .unwrap_err()
            .to_string()
        );
        assert_eq!(
            std::io::Error::new(std::io::ErrorKind::Other, "Invalid file extension").to_string(),
            actions::get_object(
                "test-error-invalid-file-extension",
                "tests/failures/test3.txt",
            )
            .unwrap_err()
            .to_string()
        );
        assert_eq!(
            std::io::Error::new(std::io::ErrorKind::Other, "Invalid file extension").to_string(),
            actions::delete_object(
                "test-error-invalid-file-extension",
                "tests/failures/test3.txt",
            )
            .unwrap_err()
            .to_string()
        )
    }
}
