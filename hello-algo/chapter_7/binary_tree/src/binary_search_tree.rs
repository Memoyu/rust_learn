use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::binary_tree::TreeNode;

type OptionTreeNodeRc = Option<Rc<RefCell<TreeNode>>>;

/// 二叉搜索树
/// 二叉搜索树的中序遍历序列是升序的
#[derive(Debug)]
pub struct BinarySearchTree {
    root: OptionTreeNodeRc,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// 查找指定值节点
    pub fn search(&self, val: i32) -> OptionTreeNodeRc {
        let mut res: OptionTreeNodeRc = None;
        let mut cur = self.root.clone();

        // 循环查找
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                // 目标值大于当前节点的值，则目标可能在右子树
                Ordering::Greater => cur = node.borrow().right.clone(),
                // 目标值小于当前节点的值，则目标可能在左子树
                Ordering::Less => cur = node.borrow().left.clone(),
                // 目标找到，终止循环
                Ordering::Equal => {
                    res = cur.clone();
                    break;
                }
            }
        }

        res
    }

    /// 插入节点
    pub fn insert(&mut self, val: i32) {
        // 如果树为空，则初始化根节点
        if self.root.is_none() {
            self.root = Some(TreeNode::new(val));
            return;
        }

        let mut cur = self.root.clone();
        // 存储插入节点的父节点
        let mut pre = None;

        // 循环查找
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                Ordering::Greater => {
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                }
                Ordering::Less => {
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                }
                Ordering::Equal => return,
            }
        }

        // 获取插入值的父节点
        let pre = pre.unwrap();
        let node = Some(TreeNode::new(val));
        // 如果目标值大于父节点的值，则放在右节点
        if val > pre.borrow().val {
            pre.borrow_mut().right = node;
        } else {
            pre.borrow_mut().left = node;
        }
    }

    /// 删除节点
    pub fn remove(&mut self, val: i32) -> Option<i32> {
        // 如果树为空，则直接返回
        if self.root.is_none() {
            return None;
        }

        let mut cur = self.root.clone();
        let mut pre = None;

        // 循环查找
        while let Some(node) = cur.clone() {
            match val.cmp(&node.borrow().val) {
                Ordering::Greater => {
                    pre = cur.clone();
                    cur = node.borrow().right.clone();
                }
                Ordering::Less => {
                    pre = cur.clone();
                    cur = node.borrow().left.clone();
                }
                Ordering::Equal => break,
            }
        }

        if cur.is_none() {
            return None;
        }

        // 获取当前节点
        let cur = cur.unwrap();
        // 解构左右节点
        let (left_child, right_child) = (cur.borrow().left.clone(), cur.borrow().right.clone());
        match (left_child.clone(), right_child.clone()) {
            // 子节点数量 = 0 or 1
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                let child = left_child.or(right_child);
                let pre = pre.unwrap();
                // 判断删除的节点是否为根节点
                if !Rc::ptr_eq(&cur, self.root.as_ref().unwrap()) {
                    // 获取删除节点的父节点的左节点
                    let left = pre.borrow().left.clone();
                    // 判断删除节点是否为左节点
                    if left.is_some() && Rc::ptr_eq(left.as_ref().unwrap(), &cur) {
                        pre.borrow_mut().left = child;
                    } else {
                        pre.borrow_mut().right = child;
                    }
                } else {
                    // 为根节点，则变更根节点
                    self.root = child
                }
            }
            (Some(_), Some(_)) => {
                // 使用右子树的值(val)最小节点替换删除节点
                // 即 获取删除节点在中序遍历中的下一个节点
                let mut temp = cur.borrow().right.clone();
                while let Some(node) = temp.clone() {
                    if node.borrow().left.is_some() {
                        temp = node.borrow().left.clone();
                    } else {
                        break;
                    }
                }

                let temp_val = temp.unwrap().borrow().val;
                // 递归删除该节点
                self.remove(temp_val);
                // 使用temp覆盖要删除的节点
                cur.borrow_mut().val = temp_val;
            }
        }

        Some(val)
    }
}
