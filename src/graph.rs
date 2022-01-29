use std::collections::VecDeque;

pub struct Graph {
    adjacency_matrix : Vec<Vec<i32>>,
    size : usize,
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            adjacency_matrix: vec![vec![0; size]; size],
            size
        }
    }

    pub fn add_edge(&mut self, v: usize, u : usize) {
        if v > self.size || u > self.size {
            return;
        }
        self.adjacency_matrix[v][u] = 1;
    }

    pub fn breadth_first_traversal(&self) -> Vec<i32> {
        let mut clock = 0;
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.size];
        let mut preorder = vec![-1; self.size];

        for i in 0..self.size {
            if !visited[i] {
                queue.push_back(i);
                self.explore(&mut clock, &mut queue, &mut visited, &mut preorder)
            }
        }
        
        preorder
    }

    
    fn explore(
        &self,
        clock: &mut i32,
        queue: &mut VecDeque<usize>,
        visited: &mut Vec<bool>,
        preorder: &mut Vec<i32>
    ) {
        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            if !visited[i] {
                visited[i] = true;
                preorder[i] = *clock;
                *clock += 1;
                for edge in self.adjacency_matrix[i].iter().enumerate() {
                    if *edge.1 == 1 && visited[edge.0] == false {
                        queue.push_back(edge.0)
                    } 
                }
            }
        }
    }

}