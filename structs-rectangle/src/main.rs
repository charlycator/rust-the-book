#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn double(size: u32) -> u32 {
        size * 2
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rectX = Rectangle {
        width: 8,
        ..rect2
    };

    dbg!(&rectX);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect2));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("My rectangle is {rect1:?}");

    dbg!(&rect1);
    dbg!(&Rectangle::double(10));
}

