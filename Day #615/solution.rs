// Stable Marriage (Gale-Shapley, men proposing). Each free man proposes down his list.
// Time: O(N^2), Space: O(N^2) for preference rank tables.
use std::collections::{HashMap, VecDeque};

fn gale_shapley(
    men: &[&str],
    guy: &HashMap<&str, Vec<&str>>,
    gal: &HashMap<&str, Vec<&str>>,
) -> HashMap<String, String> {
    let mut rank: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for (w, prefs) in gal {
        let mut r = HashMap::new();
        for (i, m) in prefs.iter().enumerate() {
            r.insert(*m, i);
        }
        rank.insert(*w, r);
    }

    let mut next: HashMap<&str, usize> = men.iter().map(|m| (*m, 0)).collect();
    let mut husband: HashMap<&str, &str> = HashMap::new();
    let mut free: VecDeque<&str> = men.iter().copied().collect();

    while let Some(m) = free.pop_front() {
        let w = guy[m][next[m]];
        *next.get_mut(m).unwrap() += 1;
        match husband.get(w) {
            None => {
                husband.insert(w, m);
            }
            Some(&cur) => {
                if rank[w][m] < rank[w][cur] {
                    husband.insert(w, m);
                    free.push_back(cur);
                } else {
                    free.push_back(m);
                }
            }
        }
    }

    let mut wife: HashMap<&str, &str> = HashMap::new();
    for (w, m) in &husband {
        wife.insert(*m, *w);
    }
    men.iter()
        .map(|m| (m.to_string(), wife[m].to_string()))
        .collect()
}

fn main() {
    let guy: HashMap<&str, Vec<&str>> = HashMap::from([
        ("andrew", vec!["caroline", "abigail", "betty"]),
        ("bill", vec!["caroline", "betty", "abigail"]),
        ("chester", vec!["betty", "caroline", "abigail"]),
    ]);
    let gal: HashMap<&str, Vec<&str>> = HashMap::from([
        ("abigail", vec!["andrew", "bill", "chester"]),
        ("betty", vec!["bill", "andrew", "chester"]),
        ("caroline", vec!["bill", "chester", "andrew"]),
    ]);
    let men = ["andrew", "bill", "chester"];
    let res = gale_shapley(&men, &guy, &gal);

    let body: Vec<String> = men
        .iter()
        .map(|m| format!("'{}': '{}'", m, res[*m]))
        .collect();
    println!("{{{}}}", body.join(", "));
}
