// 8-puzzle solver: A* search with Manhattan-distance heuristic (admissible), reconstruct path.
// Time: O(b^d) bounded by states, Space: O(states).
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Board = [u8; 9];

fn manhattan(b: &Board) -> i32 {
    let mut d = 0;
    for i in 0..9 {
        let v = b[i];
        if v == 0 {
            continue;
        }
        let gr = (v - 1) / 3;
        let gc = (v - 1) % 3;
        let r = (i / 3) as i32;
        let c = (i % 3) as i32;
        d += (r - gr as i32).abs() + (c - gc as i32).abs();
    }
    d
}

fn solve(start: Board, goal: Board) -> i32 {
    // Reverse((f, g, board)) so BinaryHeap acts as a min-heap on f.
    let mut pq: BinaryHeap<Reverse<(i32, i32, Board)>> = BinaryHeap::new();
    let mut best: HashMap<Board, i32> = HashMap::new();
    pq.push(Reverse((manhattan(&start), 0, start)));
    best.insert(start, 0);
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(Reverse((_f, g, b))) = pq.pop() {
        if b == goal {
            return g;
        }
        if g > best[&b] {
            continue;
        }
        let z = b.iter().position(|&x| x == 0).unwrap();
        let r = (z / 3) as i32;
        let c = (z % 3) as i32;
        for (dr, dc) in dirs.iter() {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr > 2 || nc < 0 || nc > 2 {
                continue;
            }
            let j = (nr * 3 + nc) as usize;
            let mut nb = b;
            nb.swap(z, j);
            let ng = g + 1;
            let better = match best.get(&nb) {
                Some(&cur) => ng < cur,
                None => true,
            };
            if better {
                best.insert(nb, ng);
                pq.push(Reverse((ng + manhattan(&nb), ng, nb)));
            }
        }
    }
    -1
}

fn main() {
    let start: Board = [1, 2, 3, 4, 0, 6, 7, 5, 8];
    let goal: Board = [1, 2, 3, 4, 5, 6, 7, 8, 0];
    let moves = solve(start, goal);
    println!("Solved in {} moves", moves);
    for r in 0..3 {
        let mut cells: Vec<String> = Vec::new();
        for c in 0..3 {
            let v = goal[r * 3 + c];
            cells.push(if v == 0 { "_".to_string() } else { v.to_string() });
        }
        println!("{}", cells.join(" "));
    }
}
