use std::collections::HashMap;

pub fn binary_search(nums: &[i32], target: i32) -> i32 {
    // nums具备有序性
    // 初始化搜索范围索引[0, n-1]
    let mut i = 0;
    let mut j = nums.len() as i32 - 1;

    // 当i > j时，搜索空间扫描完毕
    while i <= j {
        let m = i + (j - i) / 2;

        // 比对目标值
        match target.cmp(&nums[m as usize]) {
            // target 大于 m索引的值，故target位于[m + 1, j]区间内
            std::cmp::Ordering::Greater => i = m + 1,
            // target 小于 m索引的值，故target位于[i, m - 1]区间内
            std::cmp::Ordering::Less => j = m - 1,
            // target 等于 m索引的值，找到目标索引
            std::cmp::Ordering::Equal => {
                return m;
            }
        }
    }

    -1
}

/// 二分查找插入点（存在重复元素， 最左一个 target）
pub fn binary_search_insertion(nums: &[i32], target: i32) -> i32 {
    let (mut i, mut j) = (0, nums.len() as i32 - 1);
    while i <= j {
        let m = i + (j - i) / 2;
        match target.cmp(&nums[m as usize]) {
            std::cmp::Ordering::Greater => i = m + 1,
            std::cmp::Ordering::Less => j = m - 1,
            std::cmp::Ordering::Equal => j = m - 1,
        }
    }

    i
}

/// 二分查找最右一个 target
pub fn binary_search_right_edge(nums: &[i32], target: i32) -> i32 {
    // 利用查找最左元素的函数来查找最右元素
    // 方式：将查找最右一个 target 转化为查找最左一个 target + 1
    // 例如，当nums为：1, 3, 6, 6, 6, 6, 6, 10, 12, 15，target为6，而target + 1为6 + 1 = 7
    // 所以，我们需要查找的是7位置，尽管nums中没有7
    // 此时，查找最左元素的函数没有找到元素，返回的是i为10的位置（位于最右边6之后，尽管nums包含7亦是如此）
    // 最后，我们就能拿到最右一个target的位置则为返回i的前一个位置，则j = i - 1
    let i = binary_search_insertion(nums, target + 1);
    let j = i - 1;
    if j == -1 || nums[j as usize] != target {
        return -1;
    }

    j
}

pub fn two_sum_hash_table(nums: &[i32], target: i32) -> Option<Vec<i32>> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        // 判断hashmap中是否存在和为target的值
        match map.get(&(target - num)) {
            // 存在，直接返回
            Some(n) => return Some(vec![*n as i32, i as i32]),
            // 不存在，将当前值 及 索引添加到hashmap
            None => map.insert(num, i as i32),
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::binary_search::{binary_search, binary_search_insertion};

    #[test]
    fn test_binary_search() {
        let nums = [1, 3, 6, 8, 12, 15, 23, 26, 31, 35];
        let i = binary_search(&nums, 6);
        assert_eq!(i, 2);

        let i = binary_search(&nums, 26);
        assert_eq!(i, 7);

        let i = binary_search(&nums, 12);
        assert_eq!(i, 4);
    }

    #[test]
    fn test_binary_search_insertion() {
        let nums = [1, 3, 6, 6, 6, 6, 6, 10, 12, 15];
        let i = binary_search_insertion(&nums, 6);
        assert_eq!(i, 2);

        let i = binary_search_insertion(&nums, 12);
        assert_eq!(i, 8);
    }
}
