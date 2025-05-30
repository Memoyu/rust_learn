use std::collections::HashMap;

fn main() {
    let mut nums = vec![3, 4, 7, 2, 33, 5, 77, 23, 4, 6, 2, 7, 0, 43, 21, 7];
    let sum = nums.iter().sum::<i32>();
    let average = sum as f64 / nums.len() as f64;
    println!("平均值：{}", average);

    nums.sort();
    let mid = nums.len() / 2 - 1;
    println!("排序后：{:?}, 中位数：{}", nums, nums[mid]);

    let mut map = HashMap::new();
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut majority = 0;
    for (key, value) in map {
        if max_count < value {
            max_count = value;
            majority = key;
        }
    }
    println!("众数：{}", majority);
}
