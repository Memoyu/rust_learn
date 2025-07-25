use stack::{array_stack::ArrayStack, link_list_stack::LinkListStack};

fn main() {
    // 基于链表实现的栈
    let mut stack = LinkListStack::new();
    stack.push(1);
    stack.push(2);

    println!("link list stack init:{:?}", stack);
    assert_eq!(Some(2), stack.peek());

    assert_eq!(Some(2), stack.pop());

    assert_eq!(Some(1), stack.peek());

    assert_eq!(vec![1], stack.to_array());

    println!("link list stack end:{:?}", stack);

    // 基于数组实现的栈
    let mut stack = ArrayStack::new();
    stack.push(1);
    stack.push(2);

    println!("array stack init:{:?}", stack);
    assert_eq!(Some(2), stack.peek());

    assert_eq!(Some(2), stack.pop());

    assert_eq!(Some(1), stack.peek());

    assert_eq!(vec![1], stack.to_array());

    println!("array stack end:{:?}", stack)
}
