/// 键值对
#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    pub key: i32,
    pub value: String,
}

/// 基于单数组实现的哈希表
pub struct ArrayHashMap {
    buckets: Vec<Option<Pair>>,
}

impl ArrayHashMap {
    pub fn new() -> Self {
        // 初始化数组，包含100个桶
        Self {
            buckets: vec![None; 100],
        }
    }

    /// 哈希函数
    fn hash_func(&self, key: i32) -> usize {
        key as usize % 100
    }

    /// 查询键对应的值
    pub fn get(&self, key: i32) -> Option<String> {
        let index = self.hash_func(key);
        self.buckets[index].as_ref().map(|p| p.value.clone())
    }

    /// 添加键值
    pub fn put(&mut self, key: i32, val: String) {
        let index = self.hash_func(key);
        self.buckets[index] = Some(Pair {
            key: key,
            value: val,
        })
    }

    /// 删除键值
    pub fn remove(&mut self, key: i32) -> Option<String> {
        let index = self.hash_func(key);
        self.buckets[index].take().map(|p| p.value)
    }

    /// 获取所有键值对
    pub fn entry_set(&self) -> Vec<&Pair> {
        self.buckets.iter().filter_map(|p| p.as_ref()).collect()
    }

    /// 获取所有键
    pub fn key_set(&self) -> Vec<i32> {
        self.buckets
            .iter()
            .filter_map(|p| p.as_ref().map(|p| p.key))
            .collect()
    }

    /// 获取所有值
    pub fn value_set(&self) -> Vec<String> {
        self.buckets
            .iter()
            .filter_map(|p| p.as_ref().map(|p| p.value.clone()))
            .collect()
    }

    /// 打印哈希表
    pub fn print(&self) {
        for p in self.entry_set() {
            println!("{} -> {}", p.key, p.value);
        }
    }
}
