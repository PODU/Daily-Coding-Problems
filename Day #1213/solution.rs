// Day 1213: Stable marriage via Gale-Shapley (men propose).
// Each free man proposes down his list; women keep their best suitor. Time O(N^2), Space O(N^2).
use std::collections::{HashMap, VecDeque};

fn stable_match(
    guys: &HashMap<String, Vec<String>>,
    gals: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let mut rank: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for (w, prefs) in gals {
        let mut r = HashMap::new();
        for (i, m) in prefs.iter().enumerate() {
            r.insert(m.clone(), i);
        }
        rank.insert(w.clone(), r);
    }
    let mut next: HashMap<String, usize> = HashMap::new();
    let mut engaged: HashMap<String, String> = HashMap::new(); // woman -> man
    let mut free: VecDeque<String> = VecDeque::new();
    for m in guys.keys() {
        free.push_back(m.clone());
        next.insert(m.clone(), 0);
    }
    while let Some(m) = free.pop_front() {
        let idx = next[&m];
        let w = guys[&m][idx].clone();
        *next.get_mut(&m).unwrap() += 1;
        match engaged.get(&w).cloned() {
            None => {
                engaged.insert(w, m);
            }
            Some(cur) => {
                if rank[&w][&m] < rank[&w][&cur] {
                    engaged.insert(w, m);
                    free.push_back(cur);
                } else {
                    free.push_back(m);
                }
            }
        }
    }
    engaged
}

fn main() {
    let mut guys: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in [
        ("andrew", ["caroline", "abigail", "betty"]),
        ("bill", ["caroline", "betty", "abigail"]),
        ("chester", ["betty", "caroline", "abigail"]),
    ] {
        guys.insert(k.to_string(), v.iter().map(|s| s.to_string()).collect());
    }
    let mut gals: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in [
        ("abigail", ["andrew", "bill", "chester"]),
        ("betty", ["bill", "andrew", "chester"]),
        ("caroline", ["bill", "chester", "andrew"]),
    ] {
        gals.insert(k.to_string(), v.iter().map(|s| s.to_string()).collect());
    }
    let m = stable_match(&guys, &gals);
    let mut by_man: HashMap<String, String> = HashMap::new();
    for (w, man) in &m {
        by_man.insert(man.clone(), w.clone());
    }
    let mut men: Vec<&String> = guys.keys().collect();
    men.sort();
    for man in men {
        println!("{} - {}", man, by_man[man]);
    }
    // andrew - abigail / bill - caroline / chester - betty
}
