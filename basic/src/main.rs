fn main() {
    // loop
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 20 {
            break count * 2;
        }
    };

    println!("Result is {}", result);

    // for
    let list = [22, 33, 88, 66, 55];

    for item in list.iter() {
        println!("item is {}", item);
    }
}
