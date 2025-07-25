use std::{cell::RefCell, rc::Rc};

/// 链表节点
#[derive(Debug)]
struct ListNode<T> {
    val: T,                                 // 节点值
    next: Option<Rc<RefCell<ListNode<T>>>>, // 节点的下个节点
    prev: Option<Rc<RefCell<ListNode<T>>>>, // 节点的上个节点
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> Self {
        ListNode {
            val: v,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkListDeque<T> {
    front: Option<Rc<RefCell<ListNode<T>>>>,
    rear: Option<Rc<RefCell<ListNode<T>>>>,
    queue_size: usize,
}

impl<T: Copy> LinkListDeque<T> {
    pub fn new() -> Self {
        LinkListDeque {
            front: None,
            rear: None,
            queue_size: 0,
        }
    }

    /// 获取队列的长度
    pub fn size(&self) -> usize {
        self.queue_size
    }

    /// 判断队列是否为空
    pub fn is_empty(&self) -> bool {
        self.queue_size == 0
    }

    /// 入队
    fn push(&mut self, val: T, is_front: bool) {
        let node = Rc::new(RefCell::new(ListNode::new(val)));

        if is_front {
            match self.front.take() {
                // 不为空时，将节点接入头部
                Some(old_front) => {
                    old_front.borrow_mut().prev = Some(node.clone());
                    node.borrow_mut().next = Some(old_front);
                    self.front = Some(node);
                }
                // 为空时，则初始化头尾接节点
                None => {
                    self.front = Some(node.clone());
                    self.rear = Some(node)
                }
            }
        } else {
            match self.rear.take() {
                // 不为空时，将节点接入尾部
                Some(old_rear) => {
                    old_rear.borrow_mut().next = Some(node.clone());
                    node.borrow_mut().prev = Some(old_rear);
                    self.rear = Some(node);
                }
                // 为空时，则初始化头尾接节点
                None => {
                    self.front = Some(node.clone());
                    self.rear = Some(node)
                }
            }
        }
        // 更新队列长度
        self.queue_size += 1;
    }

    /// 队首入队
    pub fn push_first(&mut self, val: T) {
        self.push(val, true);
    }

    /// 队尾入队
    pub fn push_last(&mut self, val: T) {
        self.push(val, false);
    }

    /// 出队
    fn pop(&mut self, is_front: bool) -> Option<T> {
        if is_front {
            self.front.take().map(|old_front| {
                match old_front.borrow_mut().next.take() {
                    Some(new_front) => {
                        // 清空节点指向的上个点(当前要出队节点)
                        new_front.borrow_mut().prev.take();
                        self.front = Some(new_front);
                    }
                    None => {
                        // 当前要出队节点没有下个节点，则需要清空rear
                        self.rear.take();
                    }
                }
                // 更新队列长度
                self.queue_size -= 1;
                old_front.borrow().val
            })
        } else {
            self.rear.take().map(|old_rear| {
                match old_rear.borrow_mut().prev.take() {
                    Some(new_rear) => {
                        new_rear.borrow_mut().next.take();
                        self.rear = Some(new_rear);
                    }
                    None => {
                        self.front.take();
                    }
                }

                self.queue_size -= 1;
                old_rear.borrow().val
            })
        }
    }

    /// 队首出队
    pub fn pop_first(&mut self) -> Option<T> {
        self.pop(true)
    }

    /// 队尾出队
    pub fn pop_last(&mut self) -> Option<T> {
        self.pop(false)
    }

    /// 访问队首
    pub fn peek_first(&self) -> Option<T> {
        self.front.as_ref().map(|n| n.borrow().val)
    }

    /// 访问队尾
    pub fn peek_last(&self) -> Option<T> {
        self.rear.as_ref().map(|n| n.borrow().val)
    }

    /// 队列转换为数组
    pub fn to_array(&self) -> Vec<T> {
        let mut vals = Vec::<T>::new();
        fn recur<T: Copy>(cur: Option<&Rc<RefCell<ListNode<T>>>>, vals: &mut Vec<T>) {
            if let Some(cur) = cur {
                vals.push(cur.borrow().val);
                recur(cur.borrow().next.as_ref(), vals);
            }
        }
        recur(self.front.as_ref(), &mut vals);
        vals
    }
}
