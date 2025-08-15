/// 选择排序
pub fn selection_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    let n = nums.len();

    // 外循环：未排序区间[i, n-1]
    // n-1 标识最后一个元素必定是最大元素，无需再比对
    for i in 0..(n - 1) {
        let mut k = i;
        // 内循环：找到未排序区间内的最小元素
        for j in (i + 1)..n {
            // 记录最小元素的索引
            if nums[k] > nums[j] {
                k = j;
            }
        }

        // 交换元素，将最小元素添加到有序区间的尾部
        let temp = nums[k];
        nums[k] = nums[i];
        nums[i] = temp;
    }
}

/// 冒泡排序
pub fn bubble_sort(nums: &mut [i32]) {
    // 外循环：未排序的区间[0, i]
    for i in (1..nums.len()).rev() {
        // 标志位，记录本轮是否出现交换
        let mut flag = false;
        // 内循环：将未排序的[0, i]中最大的元素交换至该区域的最右边
        for j in 0..i {
            // 比对交换
            if nums[j] > nums[j + 1] {
                let temp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = temp;
                flag = true;
            }
        }

        // 如果本轮未出现交换，证明区间内已经是有序的，直接跳出
        if !flag {
            break;
        }
    }
}

/// 插入排序
pub fn insertion_sort(nums: &mut [i32]) {
    // for 循环实现
    // // 外循环：已排序的区间[0, i-1]
    // for i in 1..nums.len() {
    //     let base = nums[i];
    //     let mut j: i32 = -1; // 初始下标
    //     // 内循环：将base插入到已排序区间[0, i-1]中的正确位置
    //     for n in (0..i).rev() {
    //         // 元素右移
    //         nums[n + 1] = nums[n];
    //         // 判断当前是否为base的位置
    //         if base >= nums[n] {
    //             j = n as i32;
    //             break;
    //         }
    //     }
    //     nums[(j + 1) as usize] = base;
    // }

    // 外循环：已排序的区间[0, i-1]
    for i in 1..nums.len() {
        let (base, mut j) = (nums[i], (i - 1) as i32);

        // 内循环：将base插入到已排序区间[0, i-1]中的正确位置
        // 持续往右移动元素
        while j >= 0 && base < nums[j as usize] {
            // 移动元素
            nums[(j + 1) as usize] = nums[j as usize];
            j -= 1;
        }
        // 将base放入合适的位置
        nums[(j + 1) as usize] = base;
    }
}

/// 从三个候选元素中选取中位数
fn median_three(nums: &mut [i32], left: usize, mid: usize, right: usize) -> usize {
    let (l, m, r) = (nums[left], nums[mid], nums[right]);
    if (l <= m && m <= r) || (l >= m && m >= r) {
        return mid; //  m 在 l 和 r 之间
    }

    if (m <= l && l <= r) || (m >= l && l >= r) {
        return left; //  l 在 m 和 r 之间
    }

    //  r 在 l 和 m 之间
    right
}

/// 哨兵划分
fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    // 选取三个候选元素的中位数
    let med = median_three(nums, left, (left + right) / 2, right);
    // 将中位数交换至数组最左端
    let temp = nums[med];
    nums[med] = nums[left];
    nums[left] = temp;

    let base = nums[left];
    let mut i = left;
    let mut j = right;
    while i < j {
        // 从右向左找首个小于基准数的元素
        while j > i && nums[j] >= base {
            j -= 1;
        }

        // 从左向右找首个大于基准数的元素
        while i < j && nums[i] <= base {
            i += 1;
        }

        // 交换两个元素
        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }

    // 将基准数交换至两子数组的分界线
    nums[left] = nums[i];
    nums[i] = base;

    // 返回基准索引
    i
}

pub fn quick_sort_internal(mut left: i32, mut right: i32, nums: &mut [i32]) {
    // 子数组长度为1时终止递归
    while left < right {
        // 哨兵划分
        let pivot = partition(nums, left as usize, right as usize) as i32;
        // 对两个子数组中较短的那个执行快速排序
        if pivot - left < right - pivot {
            // 递归左子数组
            quick_sort_internal(left, pivot - 1, nums);
            // 剩余未排序区间(右子数组)为 [pivot + 1, right]
            left = pivot + 1;
        } else {
            // 递归右子数组
            quick_sort_internal(pivot + 1, right, nums);
            // 剩余未排序区间(左子数组)为 [left, pivot - 1]
            right = pivot - 1;
        }
    }
}

pub fn quick_sort(nums: &mut [i32]) {
    quick_sort_internal(0, (nums.len() - 1) as i32, nums);
}
