use std::{cell::RefCell, cmp::Ordering, rc::Rc};

type OptionTreeNodeRc = Option<Rc<RefCell<TreeNode>>>;

/// AVL树 树节点
/// 平衡二叉搜索树
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,                             // 节点值
    pub height: i32,                          // 节点高度
    pub left: Option<Rc<RefCell<TreeNode>>>,  // 左节点
    pub right: Option<Rc<RefCell<TreeNode>>>, // 右节点
}

impl TreeNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            height: 0,
            left: None,
            right: None,
        }))
    }
}

/// 平衡二叉搜索树
#[derive(Debug)]
pub struct BalancedBinarySearchTree {
    root: OptionTreeNodeRc,
}

impl BalancedBinarySearchTree {
    /// 获取节点高度
    fn height(node: OptionTreeNodeRc) -> i32 {
        // 空节点的高度为-1，叶子节点的高度为0
        match node {
            Some(node) => node.borrow().height,
            None => -1,
        }
    }

    /// 更新节点高度
    fn update_height(node: OptionTreeNodeRc) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            // 节点高度等于最高子树高度 + 1
            node.borrow_mut().height = std::cmp::max(Self::height(left), Self::height(right)) + 1
        }
    }

    /// 获取平衡因子
    /// ps: 假设平衡因子为f, 则一颗AVL树的任意节点的平衡因子皆满足 f 大于等于-1, 小于等于1(-1 <= f <= 1)
    fn balance_factor(node: OptionTreeNodeRc) -> i32 {
        match node {
            // 节点平衡因子 = 左子树高度 - 右子树高度
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
            // 空节点的平衡因子为0
            None => 0,
        }
    }

    /// 右旋操作
    /// 不考虑node节点的left or right引用问题，由调用方法处理
    fn right_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            Some(node) => {
                let child = node.borrow().left.clone().unwrap();
                let grand_child = node.borrow().right.clone();

                // 以child为原节点，将node节点向右旋转
                child.borrow_mut().right = Some(node.clone());
                node.borrow_mut().left = grand_child;

                // 更新 node、child节点的高度
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));

                // 返回旋转后子树的根节点
                Some(child)
            }
            None => None,
        }
    }

    /// 左旋操作
    fn left_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            Some(node) => {
                let child = node.borrow().right.clone().unwrap();
                let grand_child = node.borrow().left.clone();

                // 以child为原节点，将node节点向左旋转
                child.borrow_mut().left = Some(node.clone());
                node.borrow_mut().right = grand_child;

                // 更新 node、child节点的高度
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));

                // 返回旋转后子树的根节点
                Some(child)
            }
            None => None,
        }
    }

    /// 执行旋转操作，使该子树重新恢复平衡
    /// 四种旋转情况的选择条件
    /// 失衡节点的平衡因子	子节点的平衡因子	应采用的旋转方法
    ///   > 1（左偏树）        >= 0             右旋
    ///   > 1（左偏树）         < 0             先左旋后右旋
    ///   < -1（右偏树）       <= 0             左旋
    ///   < -1（右偏树）        > 0             先右旋后左旋
    fn rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        let balance_factor = Self::balance_factor(node.clone());
        // 左偏树
        if balance_factor > 1 {
            let node = node.unwrap();

            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                // 右旋
                Self::right_rotate(Some(node))
            } else {
                // 先左旋后右旋
                let left = node.borrow().left.clone();
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
        }
        // 右偏树
        else if balance_factor < -1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().right.clone()) >= 0 {
                // 左旋
                Self::left_rotate(Some(node))
            } else {
                // 先右旋后左旋
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        } else {
            None
        }
    }

    /// 递归插入节点（辅助方法）
    fn insert_internal(node: OptionTreeNodeRc, val: i32) -> OptionTreeNodeRc {
        match node {
            Some(mut node) => {
                // 1. 查找插入位置，并插入节点
                let node_val = node.borrow().val;
                match node_val.cmp(&val) {
                    // 当前节点值 大于 插入值，则往当前节点的左子树查找
                    Ordering::Greater => {
                        let left = node.borrow().left.clone();
                        node.borrow_mut().left = Self::insert_internal(left, val);
                    }
                    // 当前节点值 小于 插入值，则往当前节点的右子树查找
                    Ordering::Less => {
                        let right = node.borrow().right.clone();
                        node.borrow_mut().right = Self::insert_internal(right, val);
                    }
                    // 相等时，存在重复节点，不插入，直接返回
                    Ordering::Equal => {
                        return Some(node);
                    }
                }
                // 更新节点的高度
                Self::update_height(Some(node.clone()));
                // 2. 执行旋转操作，使子该树重新恢复平衡
                node = Self::rotate(Some(node)).unwrap();

                Some(node)
            }
            None => Some(TreeNode::new(val)),
        }
    }

    /// 插入节点
    pub fn insert(&mut self, val: i32) {
        self.root = Self::insert_internal(self.root.clone(), val);
    }

    /// 递归删除节点（辅助方法）
    fn remove_internal(node: OptionTreeNodeRc, val: i32) -> OptionTreeNodeRc {
        match node {
            Some(mut node) => {
                // 1. 查找节点并删除
                if val < node.borrow().val {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::remove_internal(left, val);
                } else if val > node.borrow().val {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::remove_internal(right, val);
                } else if node.borrow().left.is_none() || node.borrow().right.is_none() {
                    let child = node.borrow().left.clone().or(node.borrow().right.clone());
                    match child {
                        // 子节点数量 = 0 ，直接删除 node 并返回
                        None => {
                            return None;
                        }
                        // 子节点数量 = 1 ，直接删除 node
                        Some(child) => node = child,
                    }
                } else {
                    // 子节点数量 = 2 ，则将中序遍历的下个节点删除，并用该节点替换当前节点
                    let mut temp = node.borrow().right.clone().unwrap();
                    loop {
                        let temp_left = temp.borrow().left.clone();
                        if temp_left.is_none() {
                            break;
                        }

                        temp = temp_left.unwrap();
                    }

                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::remove_internal(right, temp.borrow().val);
                    node.borrow_mut().val = temp.borrow().val;
                }

                // 更新节点高度
                Self::update_height(Some(node.clone()));

                // 2. 执行旋转操作，使该子树重新恢复平衡
                node = Self::rotate(Some(node)).unwrap();

                // 返回子树根节点
                Some(node)
            }
            None => None,
        }
    }

    /// 删除节点
    pub fn remove(&mut self, val: i32) {
        Self::remove_internal(self.root.clone(), val);
    }
}
