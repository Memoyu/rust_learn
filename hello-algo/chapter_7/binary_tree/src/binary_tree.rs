use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }

    /// 层序遍历（广度优先搜索 BFS(breadth-first search)）</br>
    /// 从顶部到底部遍历二叉树，并在每一层按从左到右的顺序访问接节点
    pub fn level_order(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
        // 使用队列，遵循先进先出规则遍历
        let mut que = VecDeque::new();
        que.push_back(root.clone());
        let mut vals = Vec::new();

        while let Some(node) = que.pop_front() {
            // 出队取出val
            vals.push(node.borrow().val);

            // 先把左节点入队
            if let Some(left) = node.borrow().left.as_ref() {
                que.push_back(left.clone());
            }
            // 再把右节点入队
            if let Some(right) = node.borrow().right.as_ref() {
                que.push_back(right.clone());
            }
        }

        vals
    }

    /// 前序遍历（深度优先搜索 DFS(depth-first search)）
    pub fn pre_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = Vec::new();

        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(node) = node {
                // 访问优先级：根节点 -> 左子树 -> 右子树
                // 先取出当前节点值
                vals.push(node.borrow().val);
                // 再递归左节点
                dfs(node.borrow().left.as_ref(), vals);
                // 再递归右节点
                dfs(node.borrow().right.as_ref(), vals);
            }
        }

        dfs(root, &mut vals);
        vals
    }

    /// 中序遍历（深度优先搜索 DFS(depth-first search)）
    pub fn in_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = Vec::new();

        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(node) = node {
                // 访问优先级：左子树 -> 根节点 -> 右子树
                // 先递归左节点
                dfs(node.borrow().left.as_ref(), vals);
                // 再取出当前节点值
                vals.push(node.borrow().val);
                // 再递归右节点
                dfs(node.borrow().right.as_ref(), vals);
            }
        }

        dfs(root, &mut vals);
        vals
    }

    /// 后序遍历（深度优先搜索 DFS(depth-first search)）
    pub fn post_order(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = Vec::new();

        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(node) = node {
                // 访问优先级：左子树 -> 右子树 -> 根节点
                // 先递归左节点
                dfs(node.borrow().left.as_ref(), vals);
                // 再递归右节点
                dfs(node.borrow().right.as_ref(), vals);
                // 再取出当前节点值
                vals.push(node.borrow().val);
            }
        }

        dfs(root, &mut vals);
        vals
    }
}
