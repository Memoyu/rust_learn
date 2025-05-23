fn main() {
    // 宽度和高度作为输入，并计算出对应的长方形面积

    // 方式1
    let width1 = 30;
    let height1 = 50;

    println!(
        "1.The area of rectangle is {} square pixels",
        area_1(width1, height1)
    );

    // 方式2
    let rect1 = (30, 50);

    println!("2.The area of rectangle is {} square pixels", area_2(rect1));

    // 方式3
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "3.The area of rectangle is {} square pixels",
        area_3(&rect1)
    );
    println!("rect1 is {:?}", rect1);
}

// 方式1：直接传递参数
fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

// 方式2：元组参数
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 方式3：结构体参数
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_3(react: &Rectangle) -> u32 {
    react.width * react.height
}
