use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 1, 2, 4, 3, 2, 3, 1, 1, 3, 3, 2, 3];
    let mut hmap: HashMap<i32, i32> = HashMap::new();

    // sort the vector
    v.sort();

    // get median
    if v.len() % 2 == 0 {
        println!("median:{:#?}", v[v.len() / 2 - 1])
    } else {
        println!("median:{:#?}", v[v.len() / 2])
    }

    // create a hashmap of Int and their frequencies
    for x in &v {
        let count = hmap.entry(*x).or_insert(0);
        *count += 1;
    }

    // get a key (aka mode) for max frequency Int
    let mut max = 0;
    let mut key: i32 = 0;
    for (k, v) in &hmap {
        if max > *v {
            continue;
        } else {
            max = *v;
            key = *k
        }
    }
    println!("mode: {:#?}", key);
    println!("{:?}{:#?}", v, hmap);
}
