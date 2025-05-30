use std::collections::HashMap;

fn main() {
    // 创建HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("new: {:?}", scores);

    // 将两组数据组合成HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("collect: {:?}", scores);

    // HashMap所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 对于那些实现了Copy trait的类型会进行复制到HashMap中，而对于String这种持有所有权的值，其值将会转移且所有权会转移给哈希映射
    // 此时，field_name，field_value已经失效，尝试访问则会报错
    // println!("{}:{}", field_name, field_value);

    // 获取HashMap中指定键的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // for循环遍历HashMap
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // 更新HashMap映射
    // 覆盖旧值，使用insert
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("insert:{:?}", scores);

    // 在键没有对应值时插入数据
    // 使用entry返回的键存在与否，决定是否插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("or_insert:{:?}", scores);

    // 基于旧值更新
    // 方法or_insert实际上为我们传入的键返回了一个指向关联值的可变引用（&mut V）
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
