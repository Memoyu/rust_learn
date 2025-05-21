use std::io;

fn main() {
    // 生成一个n 阶的斐波那契数列
    // 斐波那契数列是指这样一个数列：0，1，1，2，3，5，8，13，21，34，55，89……这个数列从第3项开始 ，每一项都等于前两项之和。
    loop {
        println!("请输入需要数列的第几项：");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("非法输入");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数值");
                continue;
            }
        };

        // for
        print!("for斐波那契数列：");
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 1..=n {
            print!("{},", a);
            let t = a + b;
            a = b;
            b = t;
        }
        print!("\n\n");

        // loop
        print!("loop斐波那契数列：");
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        let mut i = 0;
        loop {
            print!("{},", a);
            let t = a + b;
            a = b;
            b = t;

            i += 1;
            if i == n {
                break;
            }
        }
        print!("\n\n");

        // while
        print!("while斐波那契数列：");
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        let mut i = n;

        while i > 0 {
            print!("{},", a);
            let t = a + b;
            a = b;
            b = t;
            i -= 1;
        }
        print!("\n\n");
    }
}
