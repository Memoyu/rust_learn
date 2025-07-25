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

/// 基于链表实现的队列
#[derive(Debug)]
pub struct LinkListQueue<T> {
    front: Option<Rc<RefCell<ListNode<T>>>>, // 头节点
    rear: Option<Rc<RefCell<ListNode<T>>>>,  // 尾节点
    queue_size: usize,                       // 队列长度
}

impl<T: Copy> LinkListQueue<T> {
    pub fn new() -> Self {
        LinkListQueue {
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
    pub fn push(&mut self, val: T) {
        let new_rear = Rc::new(RefCell::new(ListNode::new(val)));
        match self.rear.take() {
            // 存在队尾节点，队列不为空
            Some(old_rear) => {
                old_rear.borrow_mut().next = Some(new_rear.clone());
                self.rear = Some(new_rear);
            }
            // 不存在队尾节点，队列为空，需要同时初始化头节点、尾节点
            None => {
                self.front = Some(new_rear.clone());
                self.rear = Some(new_rear);
            }
        }

        self.queue_size += 1;
    }

    /// 出队
    pub fn pop(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            match old_front.borrow_mut().next.take() {
                Some(new_front) => {
                    self.front = Some(new_front);
                }
                None => {
                    self.rear.take();
                }
            }
            self.queue_size -= 1;
            old_front.borrow().val
        })
    }

    /// 访问队首元素
    pub fn peek(&self) -> Option<T> {
        self.front.as_ref().map(|f| f.borrow().val)
    }

    /// 队列转数组
    pub fn to_array(&self) -> Vec<T> {
        let mut vals = Vec::new();

        fn recur<T: Copy>(cur: Option<&Rc<RefCell<ListNode<T>>>>, res: &mut Vec<T>) {
            if let Some(cur) = cur {
                res.push(cur.borrow().val);
                recur(cur.borrow().next.as_ref(), res);
            }
        }

        recur(self.front.as_ref(), &mut vals);
        vals
    }
}
