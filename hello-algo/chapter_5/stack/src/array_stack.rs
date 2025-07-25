#[derive(Debug)]
pub struct ArrayStack<T> {
    stack: Vec<T>, // 栈数据容器
}

impl<T: Copy> ArrayStack<T> {
    pub fn new() -> Self {
        ArrayStack {
            stack: Vec::<T>::new(),
        }
    }

    /// 获取栈长度
    pub fn size(&self) -> usize {
        self.stack.len()
    }

    /// 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }

    /// 入栈
    pub fn push(&mut self, val: T) {
        self.stack.push(val);
    }

    /// 出栈
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// 访问栈顶元素
    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let v = self.stack[self.stack.len() - 1];
        Some(v)
    }

    /// 将stack转化为array
    pub fn to_array(&self) -> Vec<T> {
        self.stack.clone()
    }
}
