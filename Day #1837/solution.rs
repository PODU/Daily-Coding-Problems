// Day 1837: Stable marriage via Gale-Shapley (men propose). Always yields a stable matching.
// Time O(N^2), Space O(N^2) for preference/ranking tables.
use std::collections::{HashMap, VecDeque};

fn stable_match(
    guy_pref: &HashMap<&str, Vec<&str>>,
    gal_pref: &HashMap<&str, Vec<&str>>,
) -> HashMap<String, String> {
    let mut gal_rank: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for (gal, pref) in gal_pref {
        let mut r = HashMap::new();
        for (i, g) in pref.iter().enumerate() {
            r.insert(*g, i);
        }
        gal_rank.insert(*gal, r);
    }
    let mut next: HashMap<&str, usize> = HashMap::new();
    let mut gal_partner: HashMap<&str, &str> = HashMap::new();
    let mut free: VecDeque<&str> = VecDeque::new();
    for guy in guy_pref.keys() {
        free.push_back(*guy);
        next.insert(*guy, 0);
    }
    while let Some(guy) = free.pop_front() {
        let idx = next[guy];
        let gal = guy_pref[guy][idx];
        *next.get_mut(guy).unwrap() += 1;
        match gal_partner.get(gal) {
            None => {
                gal_partner.insert(gal, guy);
            }
            Some(&cur) => {
                if gal_rank[gal][guy] < gal_rank[gal][cur] {
                    gal_partner.insert(gal, guy);
                    free.push_back(cur);
                } else {
                    free.push_back(guy);
                }
            }
        }
    }
    gal_partner
        .iter()
        .map(|(gal, guy)| (guy.to_string(), gal.to_string()))
        .collect()
}

fn main() {
    let guy_pref: HashMap<&str, Vec<&str>> = HashMap::from([
        ("andrew", vec!["caroline", "abigail", "betty"]),
        ("bill", vec!["caroline", "betty", "abigail"]),
        ("chester", vec!["betty", "caroline", "abigail"]),
    ]);
    let gal_pref: HashMap<&str, Vec<&str>> = HashMap::from([
        ("abigail", vec!["andrew", "bill", "chester"]),
        ("betty", vec!["bill", "andrew", "chester"]),
        ("caroline", vec!["bill", "chester", "andrew"]),
    ]);
    let match_ = stable_match(&guy_pref, &gal_pref);
    let mut guys: Vec<&String> = match_.keys().collect();
    guys.sort();
    for guy in guys {
        println!("{} -> {}", guy, match_[guy]);
    }
}
