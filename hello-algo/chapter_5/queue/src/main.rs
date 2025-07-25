use crate::{array_queue::ArrayQueue, link_list_queue::LinkListQueue};

pub mod array_queue;
pub mod link_list_queue;

fn main() {
    // 基于链表实现的队列
    let mut queue = LinkListQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);

    println!("link list queue init:{:?}", queue);
    assert_eq!(Some(1), queue.peek());

    assert_eq!(Some(1), queue.pop());

    assert_eq!(Some(2), queue.peek());

    assert_eq!(vec![2, 3, 4], queue.to_array());
    println!("link list queue end:{:?}", queue);

    // 基于数组实现的队列
    let mut queue = ArrayQueue::new(4);
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);

    println!("array queue init:{:?}", queue);
    assert_eq!(Some(1), queue.peek());

    assert_eq!(Some(1), queue.pop());

    assert_eq!(Some(2), queue.peek());

    assert_eq!(vec![2, 3, 4], queue.to_array());
    println!("array queue end:{:?}", queue);
}
