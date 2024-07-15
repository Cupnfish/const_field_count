use const_field_count::FieldCount;

#[derive(FieldCount)]
struct TokenInfo {
    _name: String,
    _price: i32,
}

fn main() {
    assert_eq!(TokenInfo::field_count(), 2);
}
