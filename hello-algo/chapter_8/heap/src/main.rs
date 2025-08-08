use crate::{
    heap::{MaxHeap, MinHeap},
    top_k::top_k_heap,
};

pub mod heap;
pub mod top_k;

fn main() {
    // 大顶堆
    let mut max_heap = MaxHeap::new(vec![9, 8, 6, 6, 7, 5, 2, 1, 4, 3, 6, 2]);

    assert_eq!(max_heap.peek(), Some(9));

    max_heap.push(30);
    assert_eq!(max_heap.peek(), Some(30));

    assert_eq!(max_heap.pop(), Some(30));
    assert_eq!(max_heap.peek(), Some(9));

    max_heap.push(0);

    print!("大顶堆:");
    for i in 0..max_heap.size() {
        print!(
            "{}{}",
            if i > 0 { ", " } else { "" },
            max_heap.pop().unwrap(),
        );
    }

    println!();

    // 小顶堆
    let mut min_heap = MinHeap::new(vec![9, 8, 6, 6, 7, 5, 2, 1, 4, 3, 6, 2]);

    assert_eq!(min_heap.peek(), Some(1));

    min_heap.push(0);
    assert_eq!(min_heap.peek(), Some(0));

    assert_eq!(min_heap.pop(), Some(0));
    assert_eq!(min_heap.peek(), Some(1));

    min_heap.push(30);

    print!("小顶堆:");
    for i in 0..min_heap.size() {
        print!(
            "{}{}",
            if i > 0 { ", " } else { "" },
            min_heap.pop().unwrap(),
        );
    }

    println!();

    // top k 问题
    let nums = vec![9, 8, 6, 6, 7, 5, 2, 1, 4, 3, 6, 2];
    let mut heap = top_k_heap(nums, 4);
    print!("top k:");
    for i in 0..heap.len() {
        print!("{}{}", if i > 0 { ", " } else { "" }, heap.pop().unwrap().0,);
    }
}
