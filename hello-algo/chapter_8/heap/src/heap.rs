/// 获取左子节点索引
fn left(i: usize) -> usize {
    2 * i + 1
}

/// 获取右子节点索引
fn right(i: usize) -> usize {
    2 * i + 2
}

/// 获取父节点索引
fn parent(i: usize) -> usize {
    (i - 1) / 2
}

/// 大顶堆
pub struct MaxHeap {
    heap: Vec<i32>,
}

impl MaxHeap {
    /// 构造方法，根据输入列表建堆
    pub fn new(nums: Vec<i32>) -> Self {
        // 将列表元素原封不动添加进堆
        let mut heap = Self { heap: nums };
        // 堆化除叶节点以外的其他所有节点
        // 之所以选择倒序遍历，是因为这样能够保证当前节点之下的子树已经是合法的子堆，这样堆化当前节点才是有效的
        for i in (0..=parent(heap.size() - 1)).rev() {
            heap.heapify(i, 1);
        }
        heap
    }

    /// 获取堆大小
    pub fn size(&self) -> usize {
        self.heap.len()
    }

    /// 判断堆是否为空
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// 访问堆顶元素
    pub fn peek(&self) -> Option<i32> {
        self.heap.first().copied()
    }

    /// 交换元素
    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    /// 从i节点开始堆化</br>
    /// i 节点索引</br>
    /// f 堆化方向 0:从底至顶; 1:从顶至底
    fn heapify(&mut self, mut i: usize, f: i8) {
        if f == 0 {
            loop {
                if i == 0 {
                    break;
                }

                // 获取去父节点索引
                let p = parent(i);

                // 判断i节点值是否小于等于父节点的值，满足则终止向上堆化
                if self.heap[i] <= self.heap[p] {
                    break;
                }

                // 交换节点值
                self.swap(i, p);

                // 向上堆化
                i = p;
            }
        } else {
            loop {
                let (l, r, mut max) = (left(i), right(i), i);

                // 如果左节点存在，且左节点值大于当前最大值
                if l < self.size() && self.heap[l] > self.heap[max] {
                    max = l;
                }

                // 如果右节点存在，且右节点值大于当前最大值
                if r < self.size() && self.heap[r] > self.heap[max] {
                    max = r;
                }

                // 若节点i最大索引 l, r 越界，则无须继续堆化，跳出
                if max == i {
                    break;
                }

                // 交换两节点
                self.swap(i, max);

                // 循环向下堆化
                i = max
            }
        }
    }

    /// 插入元素
    pub fn push(&mut self, val: i32) {
        self.heap.push(val);
        // 自底至顶堆化
        self.heapify(self.size() - 1, 0);
    }

    /// 推出元素
    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // 交换堆顶与堆底元素
        self.swap(0, self.size() - 1);

        // 弹出交换后的堆顶元素
        let top_val = self.heap.pop().unwrap();

        // 自顶至底堆化
        self.heapify(0, 1);
        Some(top_val)
    }
}

/// 小顶堆
pub struct MinHeap {
    heap: Vec<i32>,
}

impl MinHeap {
    /// 构造方法，根据输入列表建堆
    pub fn new(nums: Vec<i32>) -> Self {
        // 将列表元素原封不动添加进堆
        let mut heap = Self { heap: nums };
        // 堆化除叶节点以外的其他所有节点
        for i in (0..=parent(heap.size() - 1)).rev() {
            heap.heapify(i, 1);
        }
        heap
    }

    /// 获取堆大小
    pub fn size(&self) -> usize {
        self.heap.len()
    }

    /// 判断堆是否为空
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// 访问堆顶元素
    pub fn peek(&self) -> Option<i32> {
        self.heap.first().copied()
    }

    /// 交换元素
    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    /// 从i节点开始，从底至顶堆化</br>
    /// i 节点索引</br>
    /// f 堆化方向 0:自底至顶; 1:自顶至底
    fn heapify(&mut self, mut i: usize, f: i8) {
        if f == 0 {
            loop {
                if i == 0 {
                    break;
                }

                // 获取去父节点索引
                let p = parent(i);

                // 判断i节点值是否大于等于父节点的值，满足则终止向上堆化
                if self.heap[i] >= self.heap[p] {
                    break;
                }

                // 交换节点值
                self.swap(i, p);

                // 向上堆化
                i = p;
            }
        } else {
            loop {
                let (l, r, mut max) = (left(i), right(i), i);

                // 如果左节点存在，且左节点值小于当前最大值
                if l < self.size() && self.heap[l] < self.heap[max] {
                    max = l;
                }

                // 如果右节点存在，且右节点值小于当前最大值
                if r < self.size() && self.heap[r] < self.heap[max] {
                    max = r;
                }

                // 若节点i最大索引 l, r 越界，则无须继续堆化，跳出
                if max == i {
                    break;
                }

                // 交换两节点
                self.swap(i, max);

                // 循环向下堆化
                i = max
            }
        }
    }

    /// 插入元素
    pub fn push(&mut self, val: i32) {
        self.heap.push(val);
        // 自底至顶堆化
        self.heapify(self.size() - 1, 0);
    }

    /// 推出元素
    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // 交换堆顶与堆底元素
        self.swap(0, self.size() - 1);

        // 弹出交换后的堆顶元素
        let top_val = self.heap.pop().unwrap();

        // 自顶至底堆化
        self.heapify(0, 1);
        Some(top_val)
    }
}
