/// 键值对
#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    pub key: i32,
    pub value: String,
}

/// 解决Hash冲突问题
/// 链式地址
pub struct ChainingHashMap {
    size: usize,
    capacity: usize,
    load_thres: f32,
    extend_ratio: usize,
    buckets: Vec<Vec<Pair>>,
}

impl ChainingHashMap {
    pub fn new(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity: capacity,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![vec![]; capacity],
        }
    }

    /// 哈希函数
    fn hash_func(&self, key: i32) -> usize {
        key as usize % self.capacity
    }

    /// 负载因子
    /// 计算负载因子，当负载因子大于load_thres时，则需要扩容
    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    /// 扩容hash表
    fn extend(&mut self) {
        // 暂存原hash表
        let buckets_tmp = std::mem::take(&mut self.buckets);

        // 初始化hash表
        self.capacity *= self.extend_ratio;
        self.buckets = vec![vec![]; self.capacity];
        self.size = 0;

        // 将键值对迁移到新的hash表中
        for vp in buckets_tmp {
            for p in vp {
                self.put(p.key, p.value);
            }
        }
    }

    /// 添加键值
    pub fn put(&mut self, key: i32, val: String) {
        if self.load_factor() > self.load_thres {
            self.extend();
        }

        // 获取key对应hash索引
        let index = self.hash_func(key);
        for p in self.buckets[index].iter_mut() {
            // 已存在key，则覆盖value
            if p.key == key {
                p.value = val;
                return;
            }
        }

        // 不存在则插入
        self.buckets[index].push(Pair { key, value: val });
        self.size += 1;
    }

    /// 查询操作
    pub fn get(&self, key: i32) -> Option<String> {
        // 获取key对应hash索引
        let index = self.hash_func(key);

        for p in self.buckets[index].iter() {
            // 存在，直接返回
            if p.key == key {
                return Some(p.value.clone());
            }
        }

        // 否则返回空
        None
    }

    /// 删除键值
    pub fn remove(&mut self, key: i32) -> Option<String> {
        // 获取key对应hash索引
        let index = self.hash_func(key);

        for (i, p) in self.buckets[index].iter_mut().enumerate() {
            // 存在，直接返回
            if p.key == key {
                let pair = self.buckets[index].remove(i);
                self.size -= 1;
                return Some(pair.value.clone());
            }
        }

        // 否则返回空
        None
    }

    /// 打印哈希表
    pub fn print(&self) {
        for bucket in &self.buckets {
            let mut res = Vec::new();
            for pair in bucket {
                res.push(format!("{} -> {}", pair.key, pair.value));
            }
            println!("{:?}", res);
        }
    }
}
