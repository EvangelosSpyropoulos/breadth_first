use std::collections::VecDeque;

const N: usize = 5;

fn explore(
    clock: &mut i32,
    queue: &mut VecDeque<usize>,
    edges: &[[i32; N]; N],
    visited: &mut [bool],
    preorder: &mut [i32]
) {
    while !queue.is_empty() {
        println!("{:?}", queue);
        let i = queue.pop_front().unwrap();
        if !visited[i] {
            visited[i] = true;
            *clock += 1;
            preorder[i] = *clock;
            for e in edges[i].iter().enumerate() {
                if *e.1 == 1 && visited[e.0] == false {
                    queue.push_back(e.0);
                }
            }
        }
    }
}

fn main() {
    let edges: [[i32; N]; N] = [
        [0, 1, 0, 1, 1],
        [1, 0, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ];

    let mut clock = 0;
    let mut queue = VecDeque::new();
    let mut visited = [false; N];
    let mut preorder = [-1; N];

    for i in 0..N {
        if !visited[i] {
            queue.push_back(i);
            explore(&mut clock, &mut queue, &edges, &mut visited, &mut preorder);
        }
    }
    println!("{:?}", preorder);
}
