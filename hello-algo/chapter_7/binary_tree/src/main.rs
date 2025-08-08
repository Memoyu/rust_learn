use binary_tree::{
    array_binary_tree::ArrayBinaryTree, binary_search_tree::BinarySearchTree, binary_tree as bt,
};
use bt::TreeNode;

fn main() {
    // ----------------------------------二叉树----------------------------------
    {
        // 初始化节点
        let n1 = TreeNode::new(1);
        let n2 = TreeNode::new(2);
        let n3 = TreeNode::new(3);
        let n4 = TreeNode::new(4);
        let n5 = TreeNode::new(5);
        let n6 = TreeNode::new(6);
        let n7 = TreeNode::new(7);
        // 构建节点之间的引用（指针）
        n1.borrow_mut().left = Some(n2.clone());
        n1.borrow_mut().right = Some(n3.clone());
        n2.borrow_mut().left = Some(n4);
        n2.borrow_mut().right = Some(n5);
        n3.borrow_mut().left = Some(n6);
        n3.borrow_mut().right = Some(n7);

        // 层序遍历
        let vals = TreeNode::level_order(&n1);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], vals);

        // 前序遍历
        let vals = TreeNode::pre_order(Some(&n1));
        assert_eq!(vec![1, 2, 4, 5, 3, 6, 7], vals);

        // 中序遍历
        let vals = TreeNode::in_order(Some(&n1));
        assert_eq!(vec![4, 2, 5, 1, 6, 3, 7], vals);

        // 后序遍历
        let vals = TreeNode::post_order(Some(&n1));
        assert_eq!(vec![4, 5, 2, 6, 7, 3, 1], vals);
    }

    // ----------------------------------数组表示二叉树----------------------------------
    {
        // 初始化节点
        let tree = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            None,
            None,
            Some(12),
            None,
            None,
            Some(15),
        ];
        let tree = ArrayBinaryTree::new(tree);

        // 层序遍历
        let vals = tree.level_order();
        assert_eq!(vec![1, 2, 3, 4, 6, 7, 8, 9, 12, 15], vals);

        // 前序遍历
        let vals = tree.pre_order();
        assert_eq!(vec![1, 2, 4, 8, 9, 3, 6, 12, 7, 15], vals);

        // 中序遍历
        let vals = tree.in_order();
        assert_eq!(vec![8, 4, 9, 2, 1, 12, 6, 3, 7, 15], vals);

        // 后序遍历
        let vals = tree.post_order();
        assert_eq!(vec![8, 9, 4, 2, 12, 6, 15, 7, 3, 1], vals);
    }

    //----------------------------------二叉搜索树----------------------------------
    {
        // 构建二叉搜索树
        let mut bst = BinarySearchTree::new();

        let nums = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
        for &num in &nums {
            bst.insert(num);
        }

        // 搜索节点
        let node = bst.search(14);
        assert_eq!(Some(14), node.map(|n| n.borrow().val));

        // 插入节点
        bst.insert(17);
        // println!("{:?}", bst);
        let node = bst.search(17);
        assert_eq!(Some(17), node.map(|n| n.borrow().val));

        // 移除节点
        bst.remove(4);
        let node = bst.search(4);
        assert_eq!(None, node.map(|n| n.borrow().val));
    }
}
