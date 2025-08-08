use crate::{
    adjacency_list::{GraphAdjList, Vertex},
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
    let edges = vec![
        [Vertex::from(0), Vertex::from(1)],
        [Vertex::from(0), Vertex::from(3)],
        [Vertex::from(1), Vertex::from(2)],
        [Vertex::from(2), Vertex::from(3)],
        [Vertex::from(2), Vertex::from(4)],
        [Vertex::from(3), Vertex::from(4)],
    ];
    let mut adj_list = GraphAdjList::new(edges);
    adj_list.print();
    // 添加顶点
    adj_list.add_vertex(Vertex::from(6));
    // 添加边
    adj_list.add_edge(Vertex::from(5), Vertex::from(3));
    adj_list.print();
}
