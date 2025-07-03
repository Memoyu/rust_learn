pub trait Draw {
    fn draw(&self);
}

// 按钮
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Button实现trait
impl Draw for Button {
    // 具体渲染逻辑
    fn draw(&self) {
        println!("Button is draw")
    }
}

// 文本
pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub placeholder: String,
}

// TextField实现trait
impl Draw for TextField {
    // 具体渲染逻辑
    fn draw(&self) {
        println!("TextField is draw")
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
