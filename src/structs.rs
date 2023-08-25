pub struct Table {
    name: String,
    data: Vec<(String, DataType)>,
    path: String,
}
impl Table {
    pub fn new(name: String, data: Vec<(String, DataType)>, path: String) -> Table {
        Table { name, data, path }
    }
}
pub enum DataType {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}
impl DataType {
    pub fn get_type_anotation(&self) -> &str {
        match self {
            DataType::Str(_) => "<str>",
            DataType::Int(_) => "<int>",
            DataType::Float(_) => "<float>",
            DataType::Bool(_) => "<bool>",
        }
    }
}
