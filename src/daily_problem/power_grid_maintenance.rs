use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
struct Vertex {
    id: i32,
    offline: bool,
    power_grid_id: i32,
}

impl Vertex {
    fn new(id: i32) -> Self {
        Self {
            id,
            offline: false,
            power_grid_id: -1,
        }
    }
}

struct Graph {
    adj: HashMap<i32, Vec<i32>>,
    vertices: HashMap<i32, Vertex>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj: HashMap::new(),
            vertices: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.adj.entry(u).or_default().push(v);
        self.adj.entry(v).or_default().push(u);
    }

    fn add_vertex(&mut self, id: i32, value: Vertex) {
        self.vertices.insert(id, value);
        self.adj.insert(id, Vec::new());
    }

    fn get_vertex_value(&self, id: i32) -> &Vertex {
        self.vertices.get(&id).unwrap()
    }

    fn get_vertex_value_mut(&mut self, id: i32) -> &mut Vertex {
        self.vertices.get_mut(&id).unwrap()
    }

    fn get_connected_components(&self, id: i32) -> &Vec<i32> {
        self.adj.get(&id).unwrap()
    }
}

struct Solution;

impl Solution {
    fn dfs(
        graph: &mut Graph,
        node_id: i32,
        visited: &mut HashSet<i32>,
        power_grid: &mut BinaryHeap<Reverse<i32>>,
        power_grid_id: i32,
    ) {
        if visited.contains(&node_id) {
            return;
        }

        let vertex = graph.get_vertex_value_mut(node_id);
        vertex.power_grid_id = power_grid_id;
        power_grid.push(Reverse(node_id));
        visited.insert(node_id);
        for vid in graph.get_connected_components(node_id).clone() {
            if !visited.contains(&vid) {
                Self::dfs(graph, vid, visited, power_grid, power_grid_id);
            }
        }
    }

    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = Graph::new();
        for i in 1..=c {
            graph.add_vertex(i, Vertex::new(i));
        }

        for conn in connections {
            graph.add_edge(conn[0], conn[1]);
        }

        let mut power_grid: Vec<BinaryHeap<Reverse<i32>>> = Vec::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut power_grid_id = 0;

        for i in 1..=c {
            if visited.contains(&i) {
                continue;
            }

            let mut power_grid_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
            Self::dfs(
                &mut graph,
                i,
                &mut visited,
                &mut power_grid_heap,
                power_grid_id,
            );
            power_grid.push(power_grid_heap);
            power_grid_id += 1;
        }

        let mut ans = vec![];

        for q in queries {
            let op = q[0];
            let x = q[1];

            if op == 1 {
                let vertex = graph.get_vertex_value(x);
                if !vertex.offline {
                    ans.push(x);
                } else {
                    let power_grid_id = vertex.power_grid_id as usize;
                    let mut temp_head = power_grid[power_grid_id].clone();

                    while let Some(&Reverse(top)) = temp_head.peek() {
                        let top_vertex = graph.get_vertex_value(top);
                        if top_vertex.offline {
                            temp_head.pop();
                        } else {
                            break;
                        }
                    }

                    if let Some(&Reverse(top)) = temp_head.peek() {
                        ans.push(top);
                    } else {
                        ans.push(-1);
                    }

                    power_grid[power_grid_id] = temp_head;
                }
            } else if op == 2 {
                graph.get_vertex_value_mut(x).offline = true;
            }
        }
        ans
    }
}
