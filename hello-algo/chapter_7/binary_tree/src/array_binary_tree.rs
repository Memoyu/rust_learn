/// 使用数组表示二叉树
/// 若某节点的索引为 i，则该节点的左子节点索引为 2i + 1，右子节点索引为 2i + 2
/// 子节点为空时，对应索引的值为None
pub struct ArrayBinaryTree {
    tree: Vec<Option<i32>>,
}

impl ArrayBinaryTree {
    pub fn new(arr: Vec<Option<i32>>) -> Self {
        Self { tree: arr }
    }

    /// 列表容量
    pub fn size(&self) -> usize {
        self.tree.len()
    }

    /// 获取索引为 i 节点的值
    pub fn val(&self, index: i32) -> Option<i32> {
        if index < 0 || index >= self.size() as i32 {
            None
        } else {
            self.tree[index as usize]
        }
    }

    /// 获取索引为 index 节点的左子节点的索引
    fn left_index(&self, index: i32) -> i32 {
        2 * index + 1
    }

    /// 获取索引为 index 节点的右子节点的索引
    fn right_index(&self, index: i32) -> i32 {
        2 * index + 2
    }

    /// 获取索引为 index 节点的父节点的索引
    fn parent_index(&self, index: i32) -> i32 {
        (index - 1) / 2
    }

    /// 层序遍历
    pub fn level_order(&self) -> Vec<i32> {
        self.tree.iter().filter_map(|&v| v).collect()
    }

    fn dfs(&self, index: i32, order: &str, vals: &mut Vec<i32>) {
        let val = self.val(index);
        if val.is_none() {
            return;
        }

        let val = val.unwrap();
        // 前序遍历
        if order == "pre" {
            vals.push(val);
        }
        self.dfs(self.left_index(index), order, vals);
        // 中序遍历
        if order == "in" {
            vals.push(val);
        }
        self.dfs(self.right_index(index), order, vals);
        // 后序遍历
        if order == "post" {
            vals.push(val);
        }
    }

    /// 前序遍历
    pub fn pre_order(&self) -> Vec<i32> {
        let mut vals = Vec::new();
        self.dfs(0, "pre", &mut vals);
        vals
    }

    /// 中序遍历
    pub fn in_order(&self) -> Vec<i32> {
        let mut vals = Vec::new();
        self.dfs(0, "in", &mut vals);
        vals
    }

    /// 后序遍历
    pub fn post_order(&self) -> Vec<i32> {
        let mut vals = Vec::new();
        self.dfs(0, "post", &mut vals);
        vals
    }
}
