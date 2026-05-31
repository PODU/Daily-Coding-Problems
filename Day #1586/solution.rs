// 8-puzzle solver via A* search with Manhattan-distance heuristic (admissible => optimal).
// Time: O(b^d * log) over states explored; Space: O(states) for visited/frontier.
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Board = [u8; 9];
const GOAL: Board = [1, 2, 3, 4, 5, 6, 7, 8, 0];

fn manhattan(b: &Board) -> i32 {
    let mut d = 0;
    for i in 0..9 {
        let v = b[i];
        if v == 0 { continue; }
        let gi = (v - 1) as usize;
        let dr = (i as i32 / 3 - gi as i32 / 3).abs();
        let dc = (i as i32 % 3 - gi as i32 % 3).abs();
        d += dr + dc;
    }
    d
}

fn solve(start: Board) -> Vec<u8> {
    let mut g: HashMap<Board, i32> = HashMap::new();
    let mut parent: HashMap<Board, Board> = HashMap::new();
    let mut moved: HashMap<Board, u8> = HashMap::new();
    g.insert(start, 0);
    parent.insert(start, start);
    let mut heap: BinaryHeap<Reverse<(i32, i32, Board)>> = BinaryHeap::new();
    heap.push(Reverse((manhattan(&start), 0, start)));
    let dr: [i32; 4] = [-1, 1, 0, 0];
    let dc: [i32; 4] = [0, 0, -1, 1];
    let mut found = false;
    while let Some(Reverse((_f, gc, cur))) = heap.pop() {
        if cur == GOAL { found = true; break; }
        if gc > *g.get(&cur).unwrap() { continue; }
        let z = cur.iter().position(|&x| x == 0).unwrap();
        let (zr, zc) = (z as i32 / 3, z as i32 % 3);
        for k in 0..4 {
            let nr = zr + dr[k];
            let nc = zc + dc[k];
            if nr < 0 || nr > 2 || nc < 0 || nc > 2 { continue; }
            let nz = (nr * 3 + nc) as usize;
            let mut nb = cur;
            let tile = nb[nz];
            nb[z] = tile;
            nb[nz] = 0;
            let ng = gc + 1;
            if ng < *g.get(&nb).unwrap_or(&i32::MAX) {
                g.insert(nb, ng);
                parent.insert(nb, cur);
                moved.insert(nb, tile);
                heap.push(Reverse((ng + manhattan(&nb), ng, nb)));
            }
        }
    }
    let mut tiles = Vec::new();
    if found {
        let mut cur = GOAL;
        while cur != start {
            tiles.push(*moved.get(&cur).unwrap());
            cur = *parent.get(&cur).unwrap();
        }
        tiles.reverse();
    }
    tiles
}

fn main() {
    let start: Board = [1, 2, 3, 4, 5, 6, 0, 7, 8];
    let tiles = solve(start);
    println!("Solved in {} moves", tiles.len());
    let parts: Vec<String> = tiles.iter().map(|t| t.to_string()).collect();
    println!("Tiles slid: {}", parts.join(", "));
}
