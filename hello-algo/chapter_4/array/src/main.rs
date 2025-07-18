fn main() {
    let mut nums = [1, 2, 5, 4];
    insert(&mut nums, 3, 1);

    // let mut nums = [1, 2, 5, 4];
    remove(&mut nums, 1);

    traverse(&mut nums);

    let _nums = extend(&nums, 2);
}

/// 插入元素
/// 未考虑扩容，会导致最后一个元素丢失
fn insert(nums: &mut [i32], num: i32, index: usize) {
    // 遍历交换元素，将当前index位置的元素及其后面的元素都往后移动一位
    for i in ((index + 1)..nums.len()).rev() {
        nums[i] = nums[i - 1];
    }

    nums[index] = num;
}

/// 删除元素
/// 最后一个元素未进行处理
fn remove(nums: &mut [i32], index: usize) {
    // 将需要删除的元素后的元素往前移动以为，达到覆盖的目的
    for i in index..nums.len() - 1 {
        nums[i] = nums[i + 1];
    }
    // println!("{:?}", nums);
}

/// 遍历数组
fn traverse(nums: &mut [i32]) {
    let mut _count = 0;

    // 根据索引遍历数组
    for i in 0..nums.len() {
        _count = nums[i];
    }

    // 通过迭代遍历数组
    _count = 0;
    for num in nums {
        _count = *num;
    }
}

/// 扩展数组
fn extend(nums: &[i32], enlarge: usize) -> Vec<i32> {
    // 初始化扩展后数组
    let mut res = vec![0; nums.len() + enlarge];

    // 将元素组中的元素拷贝到res中
    res[0..nums.len()].copy_from_slice(nums);

    // 等价于上述方法
    // for i in 0..nums.len() {
    //     res[i] = nums[i];
    // }

    res
}
