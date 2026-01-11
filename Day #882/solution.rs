// 8-puzzle solver via A* with Manhattan-distance heuristic (optimal moves).
// State = 9-byte array, 0 = blank. Time/space depend on solution depth.
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

const GOAL: &str = "123456780";

fn manhattan(s: &str) -> i32 {
    let b = s.as_bytes();
    let mut d = 0;
    for i in 0..9 {
        if b[i] == b'0' {
            continue;
        }
        let v = (b[i] - b'1') as i32;
        let i = i as i32;
        d += (i / 3 - v / 3).abs() + (i % 3 - v % 3).abs();
    }
    d
}

fn solve(start: &str) -> Vec<i32> {
    let mut open: BinaryHeap<Reverse<(i32, i32, String)>> = BinaryHeap::new();
    open.push(Reverse((manhattan(start), 0, start.to_string())));
    let mut g: HashMap<String, i32> = HashMap::new();
    g.insert(start.to_string(), 0);
    let mut parent: HashMap<String, (String, i32)> = HashMap::new();
    let dr = [-1i32, 1, 0, 0];
    let dc = [0i32, 0, -1, 1];
    while let Some(Reverse((_f, gc, cur))) = open.pop() {
        if cur == GOAL {
            break;
        }
        if gc > *g.get(&cur).unwrap() {
            continue;
        }
        let bytes = cur.as_bytes();
        let z = bytes.iter().position(|&ch| ch == b'0').unwrap() as i32;
        let (r, c) = (z / 3, z % 3);
        for k in 0..4 {
            let nr = r + dr[k];
            let nc = c + dc[k];
            if nr < 0 || nr > 2 || nc < 0 || nc > 2 {
                continue;
            }
            let nz = (nr * 3 + nc) as usize;
            let mut nb = bytes.to_vec();
            nb.swap(z as usize, nz);
            let nxt = String::from_utf8(nb).unwrap();
            let ng = gc + 1;
            if g.get(&nxt).map_or(true, |&old| ng < old) {
                g.insert(nxt.clone(), ng);
                let tile = (cur.as_bytes()[nz] - b'0') as i32;
                parent.insert(nxt.clone(), (cur.clone(), tile));
                open.push(Reverse((ng + manhattan(&nxt), ng, nxt)));
            }
        }
    }
    let mut moves = Vec::new();
    let mut cur = GOAL.to_string();
    while cur != start {
        let (prev, tile) = parent.get(&cur).unwrap().clone();
        moves.push(tile);
        cur = prev;
    }
    moves.reverse();
    moves
}

fn main() {
    let start = "123406758"; // [[1,2,3],[4,_,6],[7,5,8]]
    let moves = solve(start);
    println!("Solved in {} moves: {:?}", moves.len(), moves);
}
