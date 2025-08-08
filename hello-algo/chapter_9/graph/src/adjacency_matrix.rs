/// 邻接矩阵
pub struct GraphAdjMat {
    vertices: Vec<i32>,
    adj_mat: Vec<Vec<i32>>,
}

impl GraphAdjMat {
    pub fn new(vertices: Vec<i32>, edges: Vec<[usize; 2]>) -> Self {
        let mut graph = Self {
            vertices: vec![],
            adj_mat: vec![],
        };

        // 添加顶点
        for v in vertices {
            graph.add_vertex(v);
        }

        // 添加边
        for e in edges {
            graph.add_edge(e[0], e[1]);
        }

        graph
    }

    /// 获取顶点数量
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    /// 添加顶点
    pub fn add_vertex(&mut self, v: i32) {
        // 获取顶点数
        let n = self.size();
        // 添加新顶点
        self.vertices.push(v);
        // 在邻接矩阵中添加一行
        self.adj_mat.push(vec![0; n]);
        // 在邻接矩阵中添加一列
        for row in &mut self.adj_mat {
            row.push(0);
        }
    }

    /// 删除顶点
    pub fn remove_vertex(&mut self, i: usize) {
        // 越界检查
        if i > self.size() {
            panic!("index error")
        }

        // 移除顶点
        self.vertices.remove(i);
        // 删除邻接矩阵中对应i的行
        self.adj_mat.remove(i);
        // 删除邻接矩阵中对应i的列
        for row in &mut self.adj_mat {
            row.remove(i);
        }
    }

    /// 边处理
    pub fn edge_internal(&mut self, t: usize, i: usize, j: usize) {
        // 参数 t 对应操作类型，0: add; 1: remove;
        // 参数 i, j 对应 vertices 元素索引
        // 越界检查 与 边对应点是同一个
        if i > self.size() || j > self.size() || i == j {
            panic!("index error")
        }

        // 在无向图中，邻接矩阵关于主对角线对称，即满足 (i, j) == (j, i)
        self.adj_mat[i][j] = if t == 0 { 1 } else { 0 };
        self.adj_mat[j][i] = if t == 0 { 1 } else { 0 };
    }

    /// 添加边
    pub fn add_edge(&mut self, i: usize, j: usize) {
        self.edge_internal(0, i, j);
    }

    /// 删除边
    pub fn remove_edge(&mut self, i: usize, j: usize) {
        self.edge_internal(1, i, j);
    }

    /// 打印邻接矩阵
    pub fn print(&self) {
        println!("顶点列表 = {:?}", self.vertices);
        println!("邻接矩阵 =");
        println!("[");
        for row in &self.adj_mat {
            println!("  {:?},", row);
        }
        println!("]")
    }
}
