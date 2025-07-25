/// 基于数组实现的队列
/// 使用环形数组的形式规避队列元素入队或出队时，底层数组元素频繁移动造成的时间复杂度O(n)问题
#[derive(Debug)]
pub struct ArrayQueue<T> {
    queue: Vec<T>,     // 队列容器
    front: usize,      // 队首索引
    queue_size: usize, // 队列长度
}

impl<T: Copy + Default> ArrayQueue<T> {
    pub fn new(capacity: usize) -> Self {
        ArrayQueue {
            queue: vec![T::default(); capacity],
            front: 0,
            queue_size: 0,
        }
    }

    /// 队列的容量
    pub fn capacity(&self) -> usize {
        self.queue.len()
    }

    /// 队列的长度
    pub fn size(&self) -> usize {
        self.queue_size
    }

    /// 判断队列是否为空
    pub fn is_empty(&self) -> bool {
        self.queue_size == 0
    }

    /// 入队
    pub fn push(&mut self, val: T) {
        // 没有实现扩容
        if self.queue_size == self.capacity() {
            println!("队列已满");
            return;
        }

        // 计算队尾索引
        // 使用 取余操作 获得，规避rear索引溢出
        let rear = (self.front + self.queue_size) % self.capacity();
        // 元素加入队尾
        self.queue[rear] = val;
        self.queue_size += 1;
    }

    /// 出队
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let val = self.queue[self.front];

        // 与入队一样，同样使用取余法
        self.front = (self.front + 1) % self.capacity();
        self.queue_size -= 1;

        Some(val)
    }

    /// 访问队首元素
    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        Some(self.queue[self.front])
    }

    /// 队列转数组
    pub fn to_array(&self) -> Vec<T> {
        let mut vals = Vec::new();
        let mut index = self.front;
        for _ in 0..self.queue_size {
            vals.push(self.queue[index % self.capacity()]);
            index += 1;
        }

        vals
    }
}
