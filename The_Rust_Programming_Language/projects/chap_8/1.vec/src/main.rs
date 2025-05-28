fn main() {
    //-------------- 创建动态数组 --------------//
    // 使用new方法创建vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    // 使用vec!宏创建vector
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    println!("{:?}", v);

    //-------------- 获取元素 --------------//
    // 使用下标获取元素
    let third = &v[2];
    println!("The third element is {}", third);

    // 使用get方法获取元素
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    };

    // -------------- 遍历动态数组 --------------//
    // for循环
    for i in &v {
        println!("{}!", i);
    }
    // 遍历并修改元素
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // -------------- 使用枚举使得动态数组能实现存储“不同类型的值” --------------//
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.44),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
