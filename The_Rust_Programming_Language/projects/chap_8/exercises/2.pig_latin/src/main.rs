use std::io;

fn main() {
    loop {
        println!("请输入单词：");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("非法输入");

        let vowels = vec!['a', 'e', 'i', 'o', 'u'];

        let input = input.trim();
        let chars: Vec<char> = input.chars().collect();
        // println!("{:?}", chars);

        let first_char = match chars.first() {
            Some(c) => c,
            None => {
                println!("输入有误, 请重新输入");
                continue;
            }
        };

        if vowels.contains(&first_char) {
            println!("The Big Latin is: {}", format!("{}-hay", input));
        } else {
            let temp: String = chars[1..].iter().collect();
            println!("The Big Latin is: {}", format!("{}-{}ay", temp, first_char));
        }
    }
}
