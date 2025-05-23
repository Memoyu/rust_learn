#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法，且传入其他参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 调用Rectangle方法
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of rectangle is {} square pixels",
        rectangle.area()
    );

    // 调用Rectangle方法，且方法传入其他参数
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 调用调用Rectangle关联函数
    let sq = Rectangle::square(3);
    println!("The square is: {:?}", sq);
}
