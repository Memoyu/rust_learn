use gui::{Button, Draw, Screen};

// 用户自定以组件
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// 自定义组件实现Draw trait
impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox is draw")
    }
}

fn main() {
    // 构建显示
    let screen = Screen {
        components: vec![
            // 添加组件
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    // 运行渲染
    screen.run();
}
