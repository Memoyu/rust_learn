use crate::{
    adjacency_list::{GraphAdjList, Vertex, vals_to_vets},
    adjacency_matrix::GraphAdjMat,
};

pub mod adjacency_list;
pub mod adjacency_matrix;

fn main() {
    // 邻接矩阵
    let vertices = vec![1, 3, 2, 5, 4];
    let edges = vec![[0, 1], [0, 3], [1, 2], [2, 3], [2, 4], [3, 4]];
    let mut adj_mat = GraphAdjMat::new(vertices, edges);
    adj_mat.print();
    // 添加顶点
    adj_mat.add_vertex(6);
    // 添加边
    adj_mat.add_edge(5, 3);
    adj_mat.print();

    // 邻接表
    let v = vals_to_vets(vec![1, 3, 2, 5, 4, 6]);
    let edges = vec![
        [v[0], v[1]],
        [v[0], v[3]],
        [v[1], v[2]],
        [v[2], v[3]],
        [v[2], v[4]],
        [v[3], v[4]],
    ];

    let mut adj_list = GraphAdjList::new(edges);
    adj_list.print();
    // 添加顶点
    adj_list.add_vertex(v[5]);
    // 添加边
    adj_list.add_edge(v[5], v[4]);
    adj_list.print();

    let bfs = adj_list.bfs(v[1]);
    println!("{:?}", bfs);

    let dfs = adj_list.dfs(v[1]);
    println!("{:?}", dfs)
}
