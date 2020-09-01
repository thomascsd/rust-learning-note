fn main() {
    let rect1 = Rectangle::rect(5);
    let rect2 = Rectangle {
        width: 10,
        height: 5,
    };

    println!("rect1 area: {}", rect1.area());
    println!("rect2:{:?}", rect2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn rect(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
