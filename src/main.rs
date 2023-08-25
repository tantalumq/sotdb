mod actions;
mod structs;

fn main() {
    actions::create_table("example", vec!["name", "value"], "example.sotdb").unwrap();
}
