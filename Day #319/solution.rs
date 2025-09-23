// 8-puzzle solver: A* search with Manhattan-distance heuristic; blank=0.
// Time ~O(b^d) bounded by states explored; Space O(states stored).
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Board = [i32; 9];

fn manhattan(b: &Board) -> i32 {
    let mut d = 0;
    for (i, &v) in b.iter().enumerate() {
        if v == 0 { continue; }
        let (gr, gc) = ((v - 1) / 3, (v - 1) % 3);
        let (r, c) = (i as i32 / 3, i as i32 % 3);
        d += (gr - r).abs() + (gc - c).abs();
    }
    d
}

fn main() {
    let start: Board = [1, 2, 3, 4, 5, 6, 7, 0, 8];
    let goal: Board = [1, 2, 3, 4, 5, 6, 7, 8, 0];
    let moves: [(i32, &str); 4] = [(-3, "Up"), (3, "Down"), (-1, "Left"), (1, "Right")];

    let mut g: HashMap<Board, i32> = HashMap::new();
    let mut parent: HashMap<Board, (Board, String)> = HashMap::new();
    let mut pq: BinaryHeap<Reverse<(i32, i32, Board)>> = BinaryHeap::new();

    g.insert(start, 0);
    pq.push(Reverse((manhattan(&start), 0, start)));

    while let Some(Reverse((_f, gc, cur))) = pq.pop() {
        if gc > *g.get(&cur).unwrap() { continue; }
        if cur == goal { break; }
        let blank = cur.iter().position(|&x| x == 0).unwrap();
        let (r, c) = (blank as i32 / 3, blank as i32 % 3);
        for &(delta, name) in moves.iter() {
            if name == "Up" && r == 0 { continue; }
            if name == "Down" && r == 2 { continue; }
            if name == "Left" && c == 0 { continue; }
            if name == "Right" && c == 2 { continue; }
            let nb = (blank as i32 + delta) as usize;
            let mut nx = cur;
            nx.swap(blank, nb);
            let ng = gc + 1;
            if !g.contains_key(&nx) || ng < *g.get(&nx).unwrap() {
                g.insert(nx, ng);
                parent.insert(nx, (cur, name.to_string()));
                pq.push(Reverse((ng + manhattan(&nx), ng, nx)));
            }
        }
    }

    let mut path: Vec<String> = Vec::new();
    let mut cur = goal;
    while cur != start {
        let (prev, name) = parent.get(&cur).unwrap().clone();
        path.push(name);
        cur = prev;
    }
    path.reverse();
    println!("Solved in {} move(s): {}", path.len(), path.join(", "));
}
