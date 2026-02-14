// Gale-Shapley men-proposing stable matching. Time: O(N^2), Space: O(N^2).
use std::collections::{HashMap, VecDeque};

fn main() {
    let men:   Vec<&str> = vec!["andrew", "bill", "chester"];
    let women: Vec<&str> = vec!["abigail", "betty", "caroline"];
    let wi: HashMap<&str, usize> = women.iter().enumerate().map(|(i,&w)| (w,i)).collect();
    let mi: HashMap<&str, usize> = men.iter().enumerate().map(|(i,&m)| (m,i)).collect();

    let gp: Vec<Vec<usize>> = vec![
        vec![wi["caroline"], wi["abigail"], wi["betty"]],
        vec![wi["caroline"], wi["betty"],   wi["abigail"]],
        vec![wi["betty"],    wi["caroline"],wi["abigail"]],
    ];
    // gr[w][m] = preference rank of man m for woman w
    let mut gr = vec![vec![0usize; 3]; 3];
    gr[wi["abigail"]][mi["andrew"]]=0; gr[wi["abigail"]][mi["bill"]]=1; gr[wi["abigail"]][mi["chester"]]=2;
    gr[wi["betty"]][mi["bill"]]=0;     gr[wi["betty"]][mi["andrew"]]=1; gr[wi["betty"]][mi["chester"]]=2;
    gr[wi["caroline"]][mi["bill"]]=0;  gr[wi["caroline"]][mi["chester"]]=1; gr[wi["caroline"]][mi["andrew"]]=2;

    let mut wp: Vec<Option<usize>> = vec![None; 3];
    let mut nxt = vec![0usize; 3];
    let mut q: VecDeque<usize> = (0..3).collect();

    while let Some(m) = q.pop_front() {
        let w = gp[m][nxt[m]]; nxt[m] += 1;
        match wp[w] {
            None => wp[w] = Some(m),
            Some(m2) => {
                if gr[w][m] < gr[w][m2] {
                    q.push_back(m2); wp[w] = Some(m);
                } else {
                    q.push_back(m);
                }
            }
        }
    }

    let mut res: Vec<(&str, &str)> = (0..3)
        .map(|w| (men[wp[w].unwrap()], women[w]))
        .collect();
    res.sort_by_key(|p| p.0);
    for (m, w) in res {
        println!("{} -> {}", m, w);
    }
}
