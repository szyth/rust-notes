use std::collections::HashMap;
fn main() {
    let mut hmap: HashMap<&str, i32> = HashMap::new();

    let str: String = String::from("hello world yay hello");

    for word in str.split_whitespace() {
        let count = hmap.entry(word).or_insert(0);
        *count += 1;
    }
    print!("{:#?}", hmap)
}
