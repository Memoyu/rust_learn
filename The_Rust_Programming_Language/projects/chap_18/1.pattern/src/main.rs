fn main() {
    // 使用if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // 使用while let, 反复执行模式匹配
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 在for循环中使用模式来解构元组
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 此处的y覆盖了外部的y
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // 多重模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配值区间
    // 适用于 数值或char
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // match中进行解构匹配
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 匹配嵌套的枚举
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
    // 较为复杂解构
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 忽略模式中的值
    // 忽略了第一个参数
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // 忽略Some中的值，Some(_)匹配任意值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // 忽略元组中的多个部分
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // 忽略变量
    // 未使用，使用_前缀进行忽略，避免触发变量未使用警告
    let _x = 5;
    let y = 10;

    // 注意：_x语法仍然将值绑定到了变量上，而_则完全不会进行绑定
    // 所有权转移
    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s); // 此处会报错，所有权已转移
    // 不会转移
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 使用..忽略值的剩余部分
    struct Pointz {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Pointz { x: 0, y: 0, z: 0 };
    match origin {
        // 忽略x之外的所有字段
        Pointz { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 只匹配元组中的第一个值和最后一个值，而忽略其他值
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // 匹配守卫
    let num = Some(4);
    match num {
        // 在match基础上增加if x < 5匹配守卫
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    // 使用匹配守卫来解决模式中变量覆盖的问题
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 此处的y是外部变量，而n则为Some解出的值，这样就能同时匹配y，y也不会被覆盖
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 匹配守卫匹配优先级
    let x = 4;
    let y = false;
    match x {
        //  等同于：(4 | 5 | 6) if y
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // 模式中测试一个值的同时使用@来绑定它
    enum Msg {
        Hello { id: i32 },
    }
    let msg = Msg::Hello { id: 5 };
    match msg {
        // @将匹配的值绑定到了id_variable，可以捕获到匹配成功的值。
        Msg::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        // 没有储存id, 所以模式分支中无法使用id
        Msg::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // 该模式则指定了一个没有区间约束的变量，id代表该值，所以模式分支可以使用id
        Msg::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
