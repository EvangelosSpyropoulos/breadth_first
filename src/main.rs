mod graph;

use crate::graph::Graph;

fn main() {

    let mut g = Graph::new(5);
    g.add_edge(0, 1);
    g.add_edge(0, 3);
    g.add_edge(0, 4);
    g.add_edge(1, 0);
    g.add_edge(1, 3);
    g.add_edge(3, 0);
    g.add_edge(3, 1);
    g.add_edge(4, 0);

    let preorder = g.breadth_first_traversal();    
    println!("{:?}", preorder);
}
