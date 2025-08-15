use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Vertex {
    pub val: i32,
}

impl From<i32> for Vertex {
    fn from(value: i32) -> Self {
        Self { val: value }
    }
}

/* 输入值列表 vals ，返回顶点列表 vets */
pub fn vals_to_vets(vals: Vec<i32>) -> Vec<Vertex> {
    vals.into_iter().map(|val| val.into()).collect()
}

/* 输入顶点列表 vets ，返回值列表 vals */
pub fn vets_to_vals(vets: Vec<Vertex>) -> Vec<i32> {
    vets.into_iter().map(|vet| vet.val).collect()
}

/// 邻接表
pub struct GraphAdjList {
    adj_list: HashMap<Vertex, Vec<Vertex>>,
}

impl GraphAdjList {
    pub fn new(edges: Vec<[Vertex; 2]>) -> Self {
        let mut graph = Self {
            adj_list: HashMap::new(),
        };

        // 添加所有顶点和边
        for e in edges {
            graph.add_vertex(e[0]);
            graph.add_vertex(e[1]);
            graph.add_edge(e[0], e[1]);
        }

        graph
    }

    /// 获取顶点数量
    pub fn size(&self) -> usize {
        self.adj_list.len()
    }

    /// 添加顶点
    pub fn add_vertex(&mut self, v: Vertex) {
        if self.adj_list.contains_key(&v) {
            return;
        }

        self.adj_list.insert(v, vec![]);
    }

    /// 删除顶点
    pub fn remove_vertex(&mut self, v: Vertex) {
        if !self.adj_list.contains_key(&v) {
            return;
        }

        // 删除邻接表中顶点对应的链表
        self.adj_list.remove(&v);
        // 遍历其他顶点链表，删除所有包含该Vertex的边
        for list in self.adj_list.values_mut() {
            list.retain(|&lv| lv != v);
        }
    }

    /// 添加边
    pub fn add_edge(&mut self, v1: Vertex, v2: Vertex) {
        if v1 == v2 {
            panic!("value error");
        }

        // 添加边时，节点不存在，则添加顶点
        if !self.adj_list.contains_key(&v1) {
            self.add_vertex(v1);
        }

        if !self.adj_list.contains_key(&v2) {
            self.add_vertex(v2);
        }

        // 在顶点v1对应的链表中添加与v2对应的边
        self.adj_list.get_mut(&v1).unwrap().push(v2);
        // 在顶点v2对应的链表中添加与v1对应的边
        self.adj_list.get_mut(&v2).unwrap().push(v1);
    }

    /// 删除边
    pub fn remove_edge(&mut self, v1: Vertex, v2: Vertex) {
        if !self.adj_list.contains_key(&v1) || !self.adj_list.contains_key(&v2) || v1 == v2 {
            panic!("value error");
        }

        // 使用retain过滤指定节点
        // 在顶点v1对应的链表中删除与v2对应的边
        self.adj_list.get_mut(&v1).unwrap().retain(|&v| v != v2);
        // 在顶点v2对应的链表中删除与v1对应的边
        self.adj_list.get_mut(&v2).unwrap().retain(|&v| v != v1);
    }

    /// 打印邻接表
    pub fn print(&self) {
        println!("邻接表 =");
        for (vertex, list) in &self.adj_list {
            let list = list.iter().map(|vertex| vertex.val).collect::<Vec<i32>>();
            println!("{}: {:?},", vertex.val, list);
        }
    }

    /// 广度优先遍历
    pub fn bfs(&self, start_vet: Vertex) -> Vec<Vertex> {
        let mut res = Vec::new();
        // 哈希集合，用于记录已经被访问过的顶点
        let mut visited = HashSet::new();
        visited.insert(start_vet);

        // 队列，用于实现先进先出
        let mut que = VecDeque::new();
        que.push_back(start_vet);

        while !que.is_empty() {
            // 顶点出队，放入结果集
            let vet = que.pop_front().unwrap();
            res.push(vet);
            // 遍历该顶点的所有邻接顶点
            if let Some(list) = self.adj_list.get(&vet) {
                for &v in list {
                    // 已访问过则跳过
                    if visited.contains(&v) {
                        continue;
                    }

                    // 顶点入队，记录访问
                    que.push_back(v);
                    visited.insert(v);
                }
            }
        }

        res
    }

    /// 深度优先遍历
    pub fn dfs(&self, start_vet: Vertex) -> Vec<Vertex> {
        let mut res = Vec::new();
        // 哈希集合，用于记录已经被访问过的顶点
        let mut visited = HashSet::new();
        visited.insert(start_vet);

        self.dfs_internal(&mut visited, &mut res, start_vet);

        res
    }

    /// 深度优先遍历处理函数
    fn dfs_internal(&self, visited: &mut HashSet<Vertex>, res: &mut Vec<Vertex>, vet: Vertex) {
        res.push(vet);
        visited.insert(vet);

        if let Some(list) = self.adj_list.get(&vet) {
            for &v in list {
                if visited.contains(&v) {
                    continue;
                }

                // 递归访问邻接顶点
                self.dfs_internal(visited, res, v);
            }
        }
    }
}
