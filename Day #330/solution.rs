// 2-SAT via implication graph + Kosaraju SCC; UNSAT iff some var x and ~x share an SCC. O(V+E).
// Literal node = 2*var + (0 positive, 1 negative); negation = node^1. Clause (a|b): ~a->b, ~b->a.

fn node(var: usize, pos: bool) -> usize {
    2 * var + if pos { 0 } else { 1 }
}

fn dfs1(u: usize, g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, order: &mut Vec<usize>) {
    vis[u] = true;
    for &v in &g[u] {
        if !vis[v] {
            dfs1(v, g, vis, order);
        }
    }
    order.push(u);
}

fn dfs2(u: usize, c: i32, gt: &Vec<Vec<usize>>, comp: &mut Vec<i32>) {
    comp[u] = c;
    for &v in &gt[u] {
        if comp[v] == -1 {
            dfs2(v, c, gt, comp);
        }
    }
}

fn main() {
    let n_vars = 3usize; // a=0, b=1, c=2
    let n = 2 * n_vars;
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut gt: Vec<Vec<usize>> = vec![Vec::new(); n];

    // clause = ((var, pos), (var, pos))
    let clauses: Vec<((usize, bool), (usize, bool))> = vec![
        ((2, false), (1, true)),  // (~c | b)
        ((1, true), (2, true)),   // (b | c)
        ((1, false), (2, true)),  // (~b | c)
        ((2, false), (0, false)), // (~c | ~a)
    ];
    for &((v1, p1), (v2, p2)) in &clauses {
        let a = node(v1, p1);
        let b = node(v2, p2);
        g[a ^ 1].push(b);
        g[b ^ 1].push(a);
        gt[b].push(a ^ 1);
        gt[a].push(b ^ 1);
    }

    let mut vis = vec![false; n];
    let mut order = Vec::new();
    for i in 0..n {
        if !vis[i] {
            dfs1(i, &g, &mut vis, &mut order);
        }
    }
    let mut comp = vec![-1i32; n];
    let mut c = 0;
    for i in (0..n).rev() {
        let u = order[i];
        if comp[u] == -1 {
            dfs2(u, c, &gt, &mut comp);
            c += 1;
        }
    }

    let mut sat = true;
    for i in 0..n_vars {
        if comp[2 * i] == comp[2 * i + 1] {
            sat = false;
        }
    }

    let val = [false, true, true]; // a, b, c canonical
    let mut ok = true;
    for &((v1, p1), (v2, p2)) in &clauses {
        if !((val[v1] == p1) || (val[v2] == p2)) {
            ok = false;
        }
    }

    let s = |b: bool| if b { "True" } else { "False" };
    if sat && ok {
        println!("a={}, b={}, c={}", s(val[0]), s(val[1]), s(val[2]));
    } else {
        println!("UNSATISFIABLE");
    }
}
