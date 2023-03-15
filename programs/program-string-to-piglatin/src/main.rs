fn main() {
    let word_vec = vec!["first", "apple", "banana"];

    // ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"];
    let vowel_bytes: Vec<u8> = vec![97, 101, 105, 111, 117, 65, 69, 73, 79, 85];

    let mut final_str = String::new();

    for word in word_vec {
        for byte in word.bytes() {
            // if first character is a vowel, go to if case
            // first -> irst-fay
            if vowel_bytes.contains(&byte) {
                final_str = format!("{}-hay", word);
            } else {
                final_str = format!("{}-{}ay", &word[1..], byte as char);
            }
            break;
        }
        println!("{final_str}")
    }
}
