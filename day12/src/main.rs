use std::fs;

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
    let mut i = 0;
    if row > 0 {
        visit[i] = (row - 1, col);
        i += 1;
    }
    if row < graph.len() - 1 {
        visit[i] = (row + 1, col);
        i += 1;
    }
    if col > 0 {
        visit[i] = (row, col - 1);
        i += 1;
    }
    if col < graph[0].len() - 1 {
        visit[i] = (row, col + 1);
    }
    for v in visit {
        let (nrow, ncol) = v;
        if (graph[row][col] as i32 - graph[nrow][ncol] as i32) < -1 {
            // target is too high.
            continue;
        }
        if visited[nrow][ncol] {
            // already visited.
            continue;
        }
        traverse(graph, v, end, step + 1, visited, min_complete);
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
fn main() {
    let mut graph = parse();
    // println!("{:?}", graph);
    let (start, end) = startend(&mut graph);
    println!("start={:?}, end={:?}", start, end);

    // oddly enough, HashSet hashing was dominant cpu usage.
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for _r in 0..graph.len() {
        let mut row: Vec<bool> = Vec::new();
        for _c in 0..graph[0].len() {
            row.push(false);
        }
        visited.push(row);
    }
    let mut min_complete = usize::MAX;
    // let mut dbgpath: Vec<(usize, usize)> = Vec::new();
    traverse(
        &graph,
        start,
        end,
        0,
        &mut visited,
        &mut min_complete,
        //&mut dbgpath,
    );
    println!("min walk: {min_complete}");
}
