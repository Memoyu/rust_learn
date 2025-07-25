fn main() {
    // 精简版列表实现
    let mut list = List::new(3);
    // 添加
    list.add(1);
    list.add(2);
    list.add(3);
    println!("{:?}", list);

    // 更新
    list.set(2, 4);
    println!("{:?}", list);

    // 插入
    list.insert(2, 3);
    println!("{:?}", list);

    // 移除
    list.remove(0);
    println!("{:?}", list);

    assert_eq!(list.to_array(), vec![2, 3, 4]);

    // 获取元素
    println!("{:?}", list.get(1));
}

#[derive(Debug)]
struct List {
    arr: Vec<i32>,       // 数组（存储列表元素）
    capacity: usize,     // 列表容量
    size: usize,         // 列表长度（当前元素数量）
    extend_ratio: usize, // 每次列表扩容的倍数
}

impl List {
    /// 构造方法
    pub fn new(capacity: usize) -> Self {
        let vec = vec![0; capacity];
        List {
            arr: vec,
            capacity: capacity,
            size: 0,
            extend_ratio: 2,
        }
    }

    /// 获取列表长度（当前元素数量）
    pub fn size(&self) -> usize {
        self.size
    }

    /// 获取列表容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 访问元素
    pub fn get(&self, index: usize) -> i32 {
        if index >= self.size {
            panic!("索引越界")
        }
        self.arr[index]
    }

    /// 更新元素
    pub fn set(&mut self, index: usize, num: i32) {
        if index >= self.size {
            panic!("索引越界")
        }

        self.arr[index] = num
    }

    /// 在尾部添加元素
    pub fn add(&mut self, num: i32) {
        // 判断是否超出数组容量，进行扩容
        if self.size == self.capacity {
            self.extend_capacity();
        }

        // 添加元素
        self.arr[self.size] = num;
        self.size += 1
    }

    /// 在中间插入元素
    pub fn insert(&mut self, index: usize, num: i32) {
        if index >= self.size {
            panic!("索引越界")
        }
        // 判断是否超出数组容量，进行扩容
        if self.size == self.capacity {
            self.extend_capacity();
        }

        // 将index后的元素往后移动一位
        for i in (index..self.size).rev() {
            self.arr[i + 1] = self.arr[i];
        }

        // 插入新元素
        self.arr[index] = num;
        self.size += 1;
    }

    /// 删除元素
    /// 方法没有实现缩容
    pub fn remove(&mut self, index: usize) -> i32 {
        if index >= self.size {
            panic!("索引越界")
        }
        let num = self.arr[index];

        // 将index后的元素往前移动一位
        for i in index..self.size - 1 {
            self.arr[i] = self.arr[i + 1];
        }

        self.size -= 1;
        num
    }

    /// 列表扩容
    fn extend_capacity(&mut self) {
        // 计算扩容的容量
        let new_capacity = self.capacity * self.extend_ratio;

        // 构建
        let mut new_arr = vec![0; new_capacity];
        for i in 0..self.size {
            new_arr[i] = self.arr[i];
        }

        self.arr = new_arr;
        self.capacity = new_capacity;
    }

    /// 将列表转换为数组
    pub fn to_array(&self) -> Vec<i32> {
        // 仅转换有效长度范围内的列表元素
        let mut arr = Vec::new();
        for i in 0..self.size {
            arr.push(self.get(i));
        }
        arr
    }
}
