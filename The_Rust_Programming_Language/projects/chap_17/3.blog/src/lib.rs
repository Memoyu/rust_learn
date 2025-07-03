pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 添加文章内容
    pub fn add_text(&mut self, text: &str) {
        // self.state.add_text(self, text);

        if let Some(s) = self.state.take() {
            s.add_text(self, text);
            // 脱裤子放屁一下
            // 上面转移state所有权，现在又给他还回去
            self.state = Some(s);
        }
    }

    // 获取文章内容
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    // 文章请求审批
    pub fn request_review(&mut self) {
        // Option take() 会将state值取出，并使用None占位
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    // 文章审批驳回
    pub fn rejec(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.rejec());
        }
    }

    // 文章审核
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// 使用状态模式实现
// 所有规则全都在State相关的结构体中实现；
// 规则变更时，只需要调整State内部相关即可，外部调用不用变更；
// 定义状态trait
pub trait State {
    // 添加文章内容
    // 3.补充需求：用户只有在文章处于Draft状态时才能够修改文本内容（提示：将改变内容的职责从Post转移至状态对象）。
    fn add_text(&self, _post: &mut Post, _text: &str) {}

    // 请求审批
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // 驳回审批
    fn rejec(self: Box<Self>) -> Box<dyn State>;

    // 审核
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // 文章内容
    fn content<'a>(&'a self, _post: &'a Post) -> &'a str {
        ""
    }
}

// 定义草稿状态
struct Draft {}

impl State for Draft {
    fn add_text(&self, post: &mut Post, text: &str) {
        post.content.push_str(text);
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn rejec(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// 定义待审批状态
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn rejec(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PrePublish {})
    }
}

// 定义预发布状态
// 增加预发布状态，以达到两次审核的目的
struct PrePublish {}

impl State for PrePublish {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn rejec(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 定义发布状态
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn rejec(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 使用了声明周期标注'a，标注返回值的声明周期与参数post生命周期一致
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}
