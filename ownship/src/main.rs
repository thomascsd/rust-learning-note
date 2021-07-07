fn main() {
    let mut s = String::from("123");
    // let s1 = s;
    append(&mut s);

    let s1 = s;

    println!("{}", s1);
}

fn append(s: &mut String) -> &String {
    s.push_str("123");
    s
}
