use std::{cell::RefCell, rc::Rc};

/// 链表节点
#[derive(Debug)]
struct ListNode<T> {
    val: T,                                 // 节点值
    next: Option<Rc<RefCell<ListNode<T>>>>, // 节点的下个节点
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> Self {
        ListNode { val: v, next: None }
    }
}

#[derive(Debug)]
pub struct LinkListStack<T> {
    stack_peek: Option<Rc<RefCell<ListNode<T>>>>, // 将头节点作为栈顶
    stack_size: usize,                            // 栈长度
}

impl<T: Copy> LinkListStack<T> {
    pub fn new() -> Self {
        LinkListStack {
            stack_peek: None,
            stack_size: 0,
        }
    }

    /// 获取栈长度
    pub fn size(&self) -> usize {
        self.stack_size
    }

    /// 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        self.stack_size == 0
    }

    /// 入栈
    pub fn push(&mut self, val: T) {
        let mut node = ListNode::new(val);
        node.next = self.stack_peek.take();
        self.stack_peek = Some(Rc::new(RefCell::new(node)));
        self.stack_size += 1;
    }

    /// 出栈
    pub fn pop(&mut self) -> Option<T> {
        let node = self.stack_peek.take();

        node.map(|node| {
            self.stack_peek = node.borrow_mut().next.take();
            self.stack_size -= 1;

            node.borrow().val
        })

        // // 上述精简方法的拆解
        // match node {
        //     Some(node) => {
        //         self.stack_peek = node.borrow_mut().next.take();
        //         self.stack_size -= 1;

        //         Some(node.borrow().val)
        //     }
        //     None => None,
        // }
    }

    /// 访问栈顶元素
    pub fn peek(&self) -> Option<T> {
        let node = self.stack_peek.as_ref();

        node.map(|node| node.borrow().val.clone())
    }

    /// 将stack转化为array
    pub fn to_array(&self) -> Vec<T> {
        fn next_push_array<T: Copy>(node: Option<&Rc<RefCell<ListNode<T>>>>) -> Vec<T> {
            if let Some(node) = node {
                let mut vals = next_push_array(node.borrow().next.as_ref());
                vals.push(node.borrow().val);
                return vals;
            }
            return Vec::new();
        }

        next_push_array(self.stack_peek.as_ref())
    }
}
