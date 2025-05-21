use std::io;

fn main() {
    // 实现摄氏温度与华氏温度相互转换
    // 华式度 = 32 + 摄氏度 x 1.8; 摄氏度 = (华式度 - 32) / 1.8。（华式单位: °F， 摄氏单位: °C）
    loop {
        println!("选择输入的温度类型：输入C或c为摄氏温度，F或f为华氏温度，Q或q为退出");

        // 选择输入的温度类型
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("非法输入，请重新输入");

        // 匹配转换温度类型
        match unit.trim().to_uppercase().as_str() {
            "C" => {
                println!("当前为：摄氏温度 -> 华氏温度");
                let temp = get_input_temp();

                println!("摄氏温度：{}°C， 华氏温度：{}°F", temp, 32.0 + temp * 1.8);
            }
            "F" => {
                println!("当前为：华氏温度 -> 摄氏温度");
                let temp = get_input_temp();
                println!("华氏温度：{}°F， 摄氏温度：{}°C", temp, (temp - 32.0) / 1.8);
            }
            "Q" => {
                println!("退出");
                return;
            }
            _ => println!("输入的温度类型无法识别"),
        }
    }
}

// 获取温度输入
fn get_input_temp() -> f32 {
    loop {
        let mut temp = String::new();

        println!("请输入温度值：");
        io::stdin()
            .read_line(&mut temp)
            .expect("非法输入，请重新输入");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入的温度值为非数字，请重新输入！");
                continue;
            }
        };
        return temp;
    }
}
