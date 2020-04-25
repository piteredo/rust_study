fn main() {
    q1();
    q2();
}

fn q1() {
    let mut v = vec![1, 3, 5, 3, 6, 7, 5, 2, 5];
    let len = v.len();

    let mut sum = 0;
    for i in &v {
        sum += i;
    }
    let mean = sum as f64 / len as f64;
    println!("{}", mean);

    v.sort();
    let median = &v[len / 2];
    println!("{}", median);

    let mut map = std::collections::HashMap::new();
    for n in &v {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn q2() {
    let words = vec!["bird", "apple", "banana", "pink", "opus"];
    for w in &words {
        let first_letter = &w[0..1];
        let result = match first_letter {
            "a" | "i" | "u" | "e" | "o" => {
                format!("{}-hay", w)
            },
            _ => {
                let new_w = &w[1..];
                let add_w = format!("-{}ay", first_letter);
                format!("{}{}", new_w, add_w)
            }
        };
        println!("{:?}", result);
    }
}
