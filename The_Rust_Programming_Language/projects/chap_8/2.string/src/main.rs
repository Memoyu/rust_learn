fn main() {
    // 创建String
    let mut s = String::new();
    let mut s = String::from("foo");
    println!("s is {}", s);

    // 向s中添加字符串切片
    s.push_str("bar");
    println!("s.push_str is {}", s);

    // 向s中添加单个字符
    s.push('l');
    println!("s.push is {}", s);

    // 使用+拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // let s3 = &s1 + &s2; // 报错：cannot be used to concatenate two `&str` strings
    // let s3 = s1 + s2; // 报错：expected `&str`, found `String`
    let s3 = s1 + &s2; // 此时，s1已经被移动，不能再访问
    println!("{}, s2:{}", s3, s2);

    // 使用format!拼接字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // 获取字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // 遍历字符串
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }
}
