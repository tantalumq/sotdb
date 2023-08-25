mod table_actions;
mod structs;

fn main() {
    table_actions::create_table("example", vec!["name", "value"], "example.sotdb").unwrap();
}
