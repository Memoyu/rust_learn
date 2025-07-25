use std::{cell::RefCell, rc::Rc};

fn main() {
    // 初始化链表节点
    let n0 = Rc::new(RefCell::new(ListNode { val: 1, next: None }));
    let n1 = Rc::new(RefCell::new(ListNode { val: 3, next: None }));
    let n2 = Rc::new(RefCell::new(ListNode { val: 2, next: None }));
    let n3 = Rc::new(RefCell::new(ListNode { val: 5, next: None }));
    let n4 = Rc::new(RefCell::new(ListNode { val: 4, next: None }));

    // 构建节点之间的引用
    n0.borrow_mut().next = Some(n1.clone());
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());

    insert(
        &n0,
        Rc::new(RefCell::new(ListNode {
            val: 10,
            next: None,
        })),
    );

    remove(&n0);

    let _n = access(Rc::clone(&n0), 3);

    let _i = find(Rc::clone(&n0), 3);
}

/// 链表节点
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

/// 插入节点 在n节点之后插入t节点
fn insert(n: &Rc<RefCell<ListNode>>, t: Rc<RefCell<ListNode>>) {
    let next = n.borrow_mut().next.take();
    t.borrow_mut().next = next;
    n.borrow_mut().next = Some(t);
}

/// 删除节点 删除n节点后的第一个节点
fn remove(n: &Rc<RefCell<ListNode>>) {
    // n -> rn -> n1
    let rn = n.borrow_mut().next.take();
    if let Some(rn) = rn {
        let n1 = rn.borrow_mut().next.take();
        n.borrow_mut().next = n1;
    }
}

/// 访问链表指定index索引的节点
fn access(head: Rc<RefCell<ListNode>>, index: i32) -> Option<Rc<RefCell<ListNode>>> {
    fn dfs(head: Option<&Rc<RefCell<ListNode>>>, index: i32) -> Option<Rc<RefCell<ListNode>>> {
        // 递归终止条件
        if index <= 0 {
            return head.cloned();
        }

        // 下一个节点还存在，则继续递归查询
        if let Some(node) = head {
            dfs(node.borrow().next.as_ref(), index - 1)
        } else {
            None
        }
    }

    dfs(Some(head).as_ref(), index)
}

/// 查找指定的val的节点，返回节点索引
fn find(head: Rc<RefCell<ListNode>>, target: i32) -> i32 {
    fn find(head: Option<&Rc<RefCell<ListNode>>>, target: i32, index: i32) -> i32 {
        if let Some(node) = head {
            // 如果val相等，则返回索引
            if node.borrow().val == target {
                return index;
            }

            // 否则继续往后找
            return find(node.borrow().next.as_ref(), target, index + 1);
        } else {
            -1
        }
    }

    find(Some(head).as_ref(), target, 0)
}
