#[derive(Debug, Resource)]
pub struct Graph {
    pub edges: Vec<Edge>,
}

#[derive(Debug, Resource)]
pub struct Node {
    pub row: i32,
    pub col: i32,
    pub grid_cell: GridCell,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Resource)]
pub struct Edge {
    pub cost: f32,
    pub from_node: Node,
    pub to_node: Node,
}

pub impl Graph {
    pub construct_graph(grid) Graph {
        let graph = Graph{} 
    }
}
