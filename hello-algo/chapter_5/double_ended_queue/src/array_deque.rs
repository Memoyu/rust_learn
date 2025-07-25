pub struct ArrayDeque<T> {
    queue: Vec<T>,
    front: usize,
    queue_size: usize,
}

impl<T: Copy + Default> ArrayDeque<T> {
    pub fn new(capacity: usize) -> Self {
        ArrayDeque {
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

    /// 计算环形数组索引
    fn index(&self, i: i32) -> usize {
        // 通过取余操作实现数组首尾相连
        // 当 i 越过数组尾部后，回到头部
        // 当 i 越过数组头部后，回到尾部
        // i 可能为负数，所以需要先加capacity，
        ((i + self.capacity() as i32) % self.capacity() as i32) as usize
    }

    fn push(&mut self, val: T, is_front: bool) {
        if self.queue_size == self.capacity() {
            println!("双向队列已满");
            return;
        }

        if is_front {
            self.front = self.index(self.front as i32 - 1);
            self.queue[self.front] = val;
        } else {
            let rear = self.index(self.front as i32 + self.queue_size as i32);
            self.queue[rear] = val;
        }

        self.queue_size += 1;
    }

    /// 队首入队
    pub fn push_first(&mut self, val: T) {
        self.push(val, true);
    }

    /// 队尾入队
    pub fn push_last(&mut self, val: T) {
        self.push(val, false);
    }

    /// 队首出队
    pub fn pop_first(&mut self) -> Option<T> {
        let val = self.peek(true);
        self.front = self.index(self.front as i32 + 1);
        self.queue_size -= 1;
        val
    }

    /// 队尾出队
    pub fn pop_last(&mut self) -> Option<T> {
        let val = self.peek(false);
        self.queue_size -= 1;
        val
    }

    fn peek(&self, is_front: bool) -> Option<T> {
        if self.is_empty() {
            return None;
        };

        if is_front {
            Some(self.queue[self.front])
        } else {
            let rear = self.index(self.front as i32 + self.queue_size as i32 - 1);
            let val = self.queue[rear];
            Some(val)
        }
    }

    /// 访问队首
    pub fn peek_first(&self) -> Option<T> {
        self.peek(true)
    }

    /// 访问队尾
    pub fn peek_last(&self) -> Option<T> {
        self.peek(false)
    }

    /// 队列转换为数组
    pub fn to_array(&self) -> Vec<T> {
        let mut vals = Vec::<T>::new();
        let mut index = self.front;
        for _i in 0..self.queue_size {
            vals.push(self.queue[self.index(index as i32)]);
            index += 1;
        }

        vals
    }
}
