use derive_macro::IntoStringHashmap;

#[derive(IntoStringHashmap)]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    age: u32,
}
fn main() {
    println!("Hello, world!");
    for i in 0..23 {
        println!("{}", i);
    }
}
