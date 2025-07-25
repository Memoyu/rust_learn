use std::collections::VecDeque;

use crate::{array_deque::ArrayDeque, link_list_deque::LinkListDeque};

pub mod array_deque;
pub mod link_list_deque;

fn main() {
    // 基于链表实现的双向队列
    let mut queue = LinkListDeque::new();
    queue.push_first(2);
    queue.push_last(3);
    queue.push_first(1);
    queue.push_last(4);

    assert_eq!(Some(1), queue.peek_first());
    assert_eq!(Some(1), queue.pop_first());
    assert_eq!(Some(4), queue.pop_last());
    assert_eq!(Some(3), queue.peek_last());
    assert_eq!(vec![2, 3], queue.to_array());

    // 基于数组实现的双向队列
    let mut queue = ArrayDeque::new(4);
    queue.push_first(2);
    queue.push_last(3);
    queue.push_first(1);
    queue.push_last(4);

    assert_eq!(Some(1), queue.peek_first());
    assert_eq!(Some(1), queue.pop_first());
    assert_eq!(Some(4), queue.pop_last());
    assert_eq!(Some(3), queue.peek_last());
    assert_eq!(vec![2, 3], queue.to_array());

    // let mut vd = VecDeque::new();
    // vd.push_front(2);
}
