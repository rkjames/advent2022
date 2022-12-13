use petgraph::algo;
use petgraph::graph::Graph;
use std::{cmp, fs};

fn parse() -> Vec<Vec<char>> {
    let data = fs::read_to_string("test1.txt").expect("read to string failed");
    let mut v: Vec<Vec<char>> = Vec::new();
    let lines: Vec<&str> = data.split("\n").collect();

    for line in lines {
        let row: Vec<char> = line.trim().chars().collect();
        v.push(row);
    }

    v
}

// traverses graph, at location
// skips, and updates, visited
// updates min_complete to shortest discovered path
fn traverse(
    graph: &Vec<Vec<char>>,
    location: (usize, usize),
    end: (usize, usize),
    step: usize,
    visited: &mut Vec<Vec<bool>>,
    min_complete: &mut usize,
) {
    let (row, col) = location;

    if location == end {
        // finished.
        if step < *min_complete {
            println!(
                "{:?}, {row}, {col}, {}, step {step}, min_complete: {min_complete}",
                location, graph[row][col]
            );
            *min_complete = step;
        }
        return;
    }

    visited[row][col] = true;

    // avoid heap allocation.
    let mut visit = [(0, 0), (0, 0), (0, 0), (0, 0)];
    let mut c = 0;
    if row < graph.len() - 1 {
        visit[c] = (row + 1, col);
        c += 1;
    }
    if row > 0 {
        visit[c] = (row - 1, col);
        c += 1;
    }
    if col < graph[0].len() - 1 {
        visit[c] = (row, col + 1);
        c += 1;
    }
    if col > 0 {
        visit[c] = (row, col - 1);
        c += 1;
    }
    for i in 0..c {
        let (nrow, ncol) = visit[i];
        if (graph[row][col] as i32 - graph[nrow][ncol] as i32) < -1 {
            // target is too high.
            continue;
        }
        if visited[nrow][ncol] {
            // already visited.
            continue;
        }
        traverse(graph, visit[i], end, step + 1, visited, min_complete);
    }

    visited[row][col] = false;
}

// finds start and end, updates value to a/z, returns locations
fn startend(graph: &mut Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..graph.len() {
        for col in 0..graph[row].len() {
            if graph[row][col] == 'S' {
                graph[row][col] = 'a';
                start = (row, col);
            }
            if graph[row][col] == 'E' {
                graph[row][col] = 'z';
                end = (row, col);
            }
        }
    }
    (start, end)
}

fn build_petgraph(ingraph: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    let mut graph = Graph::<(), ()>::new(); // directed and unlabeled
    let mut nodes: Vec<Vec<_>> = Vec::new();

    // vertices
    for row in 0..ingraph.len() {
        let mut nodes_row: Vec<_> = Vec::new();
        for _col in 0..ingraph[row].len() {
            let node_index = graph.add_node(());
            nodes_row.push(node_index);
        }
        nodes.push(nodes_row);
    }

    // edges
    for row in 0..ingraph.len() {
        for col in 0..ingraph[row].len() {
            let mut visit = [(0, 0), (0, 0), (0, 0), (0, 0)];
            let mut c = 0;
            if row < ingraph.len() - 1 {
                visit[c] = (row + 1, col);
                c += 1;
            }
            if row > 0 {
                visit[c] = (row - 1, col);
                c += 1;
            }
            if col < ingraph[0].len() - 1 {
                visit[c] = (row, col + 1);
                c += 1;
            }
            if col > 0 {
                visit[c] = (row, col - 1);
                c += 1;
            }
            for i in 0..c {
                let (nrow, ncol) = visit[i];
                if (ingraph[row][col] as i32 - ingraph[nrow][ncol] as i32) < -1 {
                    // target is too high.
                    continue;
                }
                graph.add_edge(nodes[row][col], nodes[nrow][ncol], ());
            }
        }
    }

    // dijkstra
    let (r, c) = start;
    let start_index = nodes[r][c];
    let result = algo::dijkstra(&graph, start_index, None, |_| 1);
    let (r, c) = end;
    let end_index = nodes[r][c];
    println!(
        "part 1: cost from {:?} to {:?} is {}",
        start,
        end,
        result.get(&end_index).unwrap()
    );

    let mut cheapest = 999999;
    for row in 0..ingraph.len() {
        for col in 0..ingraph[row].len() {
            if ingraph[row][col] == 'a' {
                let start_index = nodes[row][col];
                let result = algo::dijkstra(&graph, start_index, None, |_| 1);
                let steps_maybe = result.get(&end_index);
                if steps_maybe.is_some() {
                    let steps = steps_maybe.unwrap();
                    cheapest = cmp::min(cheapest, *steps);
                }
            }
        }
    }

    println!("cheapest = {cheapest}");
}

fn main() {
    let mut graph = parse();
    // println!("{:?}", graph);
    let (start, end) = startend(&mut graph);
    println!("start={:?}, end={:?}", start, end);
    build_petgraph(graph, start, end);

    // oddly enough, HashSet hashing was dominant cpu usage.
    // let mut visited: Vec<Vec<bool>> = Vec::new();
    // for _r in 0..graph.len() {
    //     let mut row: Vec<bool> = Vec::new();
    //     for _c in 0..graph[0].len() {
    //         row.push(false);
    //     }
    //     visited.push(row);
    // }
    // let mut min_complete = usize::MAX;
    // // let mut dbgpath: Vec<(usize, usize)> = Vec::new();
    // traverse(
    //     &graph,
    //     start,
    //     end,
    //     0,
    //     &mut visited,
    //     &mut min_complete,
    //     //&mut dbgpath,
    // );
    // println!("min walk: {min_complete}");
}
