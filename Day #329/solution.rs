// Gale-Shapley stable marriage, men propose; free man proposes down his list, women keep better partner.
// Time O(n^2), Space O(n^2).
use std::collections::{HashMap, VecDeque};

fn main() {
    let men = vec!["andrew", "bill", "chester"];
    let women = vec!["abigail", "betty", "caroline"];
    let mut guy_pref: HashMap<&str, Vec<&str>> = HashMap::new();
    guy_pref.insert("andrew", vec!["caroline", "abigail", "betty"]);
    guy_pref.insert("bill", vec!["caroline", "betty", "abigail"]);
    guy_pref.insert("chester", vec!["betty", "caroline", "abigail"]);
    let mut gal_pref: HashMap<&str, Vec<&str>> = HashMap::new();
    gal_pref.insert("abigail", vec!["andrew", "bill", "chester"]);
    gal_pref.insert("betty", vec!["bill", "andrew", "chester"]);
    gal_pref.insert("caroline", vec!["bill", "chester", "andrew"]);

    let mut wrank: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    for w in &women {
        let mut r = HashMap::new();
        for (i, m) in gal_pref[w].iter().enumerate() {
            r.insert(*m, i);
        }
        wrank.insert(*w, r);
    }

    let mut next: HashMap<&str, usize> = HashMap::new();
    for m in &men {
        next.insert(*m, 0);
    }
    let mut partner_of: HashMap<&str, &str> = HashMap::new(); // woman -> man
    let mut free_men: VecDeque<&str> = men.iter().cloned().collect();

    while let Some(m) = free_men.pop_front() {
        let idx = next[m];
        next.insert(m, idx + 1);
        let w = guy_pref[m][idx];
        if !partner_of.contains_key(w) {
            partner_of.insert(w, m);
        } else {
            let cur = partner_of[w];
            if wrank[w][m] < wrank[w][cur] {
                partner_of.insert(w, m);
                free_men.push_back(cur);
            } else {
                free_men.push_back(m);
            }
        }
    }

    let mut man_to_woman: HashMap<&str, &str> = HashMap::new();
    for (woman, man) in &partner_of {
        man_to_woman.insert(*man, *woman);
    }
    for m in &men {
        println!("{} - {}", m, man_to_woman[m]);
    }
}
