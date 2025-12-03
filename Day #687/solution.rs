// 8-puzzle solver via A* with Manhattan-distance heuristic (admissible -> optimal solution).
// State = [u8;9] board (blank=0); explore by sliding a tile into the blank.
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Board = [u8; 9];
const GOAL: Board = [1, 2, 3, 4, 5, 6, 7, 8, 0];

fn manhattan(b: &Board) -> i32 {
    let mut d = 0;
    for i in 0..9 {
        let v = b[i];
        if v == 0 { continue; }
        let (gr, gc) = (((v - 1) / 3) as i32, ((v - 1) % 3) as i32);
        let (r, c) = ((i / 3) as i32, (i % 3) as i32);
        d += (gr - r).abs() + (gc - c).abs();
    }
    d
}

fn neighbors(b: &Board) -> Vec<Board> {
    let z = b.iter().position(|&x| x == 0).unwrap();
    let (zr, zc) = ((z / 3) as i32, (z % 3) as i32);
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut res = Vec::new();
    for (dr, dc) in dirs {
        let (nr, nc) = (zr + dr, zc + dc);
        if nr >= 0 && nr < 3 && nc >= 0 && nc < 3 {
            let nz = (nr * 3 + nc) as usize;
            let mut nb = *b;
            nb.swap(z, nz);
            res.push(nb);
        }
    }
    res
}

fn main() {
    let start: Board = [1, 2, 3, 4, 0, 6, 7, 5, 8];
    let mut g: HashMap<Board, i32> = HashMap::new();
    let mut parent: HashMap<Board, Board> = HashMap::new();
    g.insert(start, 0);
    let mut pq: BinaryHeap<Reverse<(i32, Board)>> = BinaryHeap::new();
    pq.push(Reverse((manhattan(&start), start)));

    while let Some(Reverse((f, cur))) = pq.pop() {
        if cur == GOAL { break; }
        if f > g[&cur] + manhattan(&cur) { continue; } // stale entry
        let cg = g[&cur];
        for nb in neighbors(&cur) {
            let ng = cg + 1;
            if !g.contains_key(&nb) || ng < g[&nb] {
                g.insert(nb, ng);
                parent.insert(nb, cur);
                pq.push(Reverse((ng + manhattan(&nb), nb)));
            }
        }
    }

    let mut path: Vec<Board> = Vec::new();
    let mut cur = GOAL;
    loop {
        path.push(cur);
        if cur == start { break; }
        cur = parent[&cur];
    }
    path.reverse();

    let moves = path.len() - 1;
    println!("Solved in {} moves", moves);
    println!("Goal board:");
    let gb = path[path.len() - 1];
    for i in 0..3 {
        println!("{} {} {}", gb[i * 3], gb[i * 3 + 1], gb[i * 3 + 2]);
    }
}
