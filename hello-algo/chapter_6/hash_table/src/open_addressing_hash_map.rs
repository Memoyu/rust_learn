/// 键值对
#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    pub key: i32,
    pub value: String,
}

pub struct OpenAddressingHashMap {
    size: usize,                // 键值对数量
    capacity: usize,            // 哈希表容量
    load_thres: f32,            // 触发扩容的负载因子阈值
    extend_ratio: usize,        // 扩容倍数
    buckets: Vec<Option<Pair>>, // 桶数组
    TOMBSTONE: Option<Pair>,    // 删除标记
}

impl OpenAddressingHashMap {
    pub fn new(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity: capacity,
            load_thres: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![None; capacity],
            TOMBSTONE: Some(Pair {
                key: -1,
                value: "-1".to_string(),
            }),
        }
    }

    /// 哈希函数
    fn hash_func(&self, key: i32) -> usize {
        (key % self.capacity as i32) as usize
    }

    /// 负载因子
    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    /// 搜索 key 对应的桶索引
    fn find_bucket(&mut self, key: i32) -> usize {
        let mut index = self.hash_func(key);
        let mut first_tombstone = -1;

        // 线性探测，当遇到空bucket时跳出
        while self.buckets[index].is_some() {
            if self.buckets[index].as_ref().unwrap().key == key {
                // 若在此之前遇到了删除标记，则将该键值对移至first_tombstone索引处
                if first_tombstone != -1 {
                    self.buckets[first_tombstone as usize] = self.buckets[index].take();
                    self.buckets[index] = self.TOMBSTONE.clone();
                    return first_tombstone as usize;
                }
                return index;
            }

            // 记录遇到的首个删除标记
            if first_tombstone == -1 && self.buckets[index] == self.TOMBSTONE {
                first_tombstone = index as i32;
            }

            // 计算bucket索引
            // 使用取余，在超出数组容量时返回头部
            index = (index + 1) % self.capacity;
        }

        // 若 key 不存在，则返回添加点的索引
        if first_tombstone == -1 {
            index
        } else {
            first_tombstone as usize
        }
    }

    /// 扩容hash表
    fn extend(&mut self) {
        // 暂存原hash表
        let buckets_tmp = std::mem::take(&mut self.buckets);

        // 初始化hash表
        self.capacity *= self.extend_ratio;
        self.buckets = vec![None; self.capacity];
        self.size = 0;

        // 将键值对迁移到新的hash表中
        for p in buckets_tmp {
            if p.is_none() || p == self.TOMBSTONE {
                continue;
            }

            let p = p.unwrap();
            self.put(p.key, p.value);
        }
    }

    /// 添加键值
    pub fn put(&mut self, key: i32, val: String) {
        // 当负载因子超过阈值时，执行扩容
        if self.load_factor() > self.load_thres {
            self.extend();
        }

        // 搜索key对应的bucket索引
        let index = self.find_bucket(key);
        if self.buckets[index].is_some() && self.buckets[index] != self.TOMBSTONE {
            self.buckets[index].as_mut().unwrap().value = val;
            return;
        }

        // 键值不存在
        self.buckets[index] = Some(Pair {
            key: key,
            value: val,
        });
        self.size += 1;
    }

    /// 查询操作
    pub fn get(&mut self, key: i32) -> Option<String> {
        // 搜索key对应的bucket索引
        let index = self.find_bucket(key);
        if self.buckets[index].is_some() && self.buckets[index] != self.TOMBSTONE {
            return self.buckets[index].as_ref().map(|p| p.value.clone());
        }

        // 键值不存在
        None
    }

    /// 删除操作
    pub fn remove(&mut self, key: i32) -> Option<String> {
        // 搜索key对应的bucket索引
        let index = self.find_bucket(key);
        if self.buckets[index].is_some() && self.buckets[index] != self.TOMBSTONE {
            let val = self.buckets[index].as_ref().map(|p| p.value.clone());
            self.buckets[index] = self.TOMBSTONE.clone();
            self.size -= 1;
            return val;
        }

        // 键值不存在
        None
    }

    /// 打印哈希表
    fn print(&self) {
        for pair in &self.buckets {
            if pair.is_none() {
                println!("null");
            } else if pair == &self.TOMBSTONE {
                println!("TOMBSTONE");
            } else {
                let pair = pair.as_ref().unwrap();
                println!("{} -> {}", pair.key, pair.value);
            }
        }
    }
}
