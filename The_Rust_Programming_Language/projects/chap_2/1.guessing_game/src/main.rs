use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 生成随机的密钥值
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // 获取输入
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 处理非法输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        // 比较输入值与生成的密钥值大小，并给出提示
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        };
    }
}
