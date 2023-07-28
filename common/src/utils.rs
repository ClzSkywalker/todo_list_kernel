use rusty_ulid::Ulid;

pub fn generate_ulid()->String{
    Ulid::generate().to_string()
}