use std::{cmp::Reverse, collections::BinaryHeap};

/// 基于堆查找数组中最大的 k 个元素
pub fn top_k_heap(nums: Vec<i32>, k: usize) -> BinaryHeap<Reverse<i32>> {
    // BinaryHeap 是大顶堆，使用 Reverse 将元素取反，从而实现小顶堆
    let mut heap = BinaryHeap::<Reverse<i32>>::new();

    // 将数组的前 k 个元素入堆
    for &num in nums.iter().take(k) {
        heap.push(Reverse(num));
    }

    // 从第 k+1 个元素开始，保持堆的长度为 k
    for &num in nums.iter().skip(k) {
        // 若当前元素大于堆顶元素，则将堆顶元素出堆、当前元素入堆
        if num > heap.peek().unwrap().0 {
            heap.pop();
            heap.push(Reverse(num));
        }
    }

    heap
}
