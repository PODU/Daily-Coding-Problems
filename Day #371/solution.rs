// Day 371: Solve a system of addition-only equations over variables/constants.
// Build a linear system A x = b and run Gauss-Jordan elimination; unique integer
// solution -> mapping, otherwise null. Time O(eqs * vars^2).
use std::collections::{BTreeSet, HashMap};

fn is_number(t: &str) -> bool {
    if t.is_empty() {
        return false;
    }
    let bytes = t.as_bytes();
    let start = if bytes[0] == b'-' { 1 } else { 0 };
    if start == t.len() {
        return false;
    }
    bytes[start..].iter().all(|&c| c.is_ascii_digit())
}

fn solve(text: &str) -> Option<Vec<(String, i64)>> {
    let mut eqs: Vec<(HashMap<String, f64>, f64)> = Vec::new();
    let mut varset: BTreeSet<String> = BTreeSet::new();
    for raw in text.split('\n') {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let sides: Vec<&str> = line.splitn(2, '=').collect();
        let mut coeffs: HashMap<String, f64> = HashMap::new();
        let mut b = 0.0;
        for tok in sides[0].split('+') {
            let t = tok.trim();
            if is_number(t) {
                b -= t.parse::<i64>().unwrap() as f64;
            } else {
                *coeffs.entry(t.to_string()).or_insert(0.0) += 1.0;
                varset.insert(t.to_string());
            }
        }
        for tok in sides[1].split('+') {
            let t = tok.trim();
            if is_number(t) {
                b += t.parse::<i64>().unwrap() as f64;
            } else {
                *coeffs.entry(t.to_string()).or_insert(0.0) -= 1.0;
                varset.insert(t.to_string());
            }
        }
        eqs.push((coeffs, b));
    }

    let vars: Vec<String> = varset.into_iter().collect();
    let idx: HashMap<&String, usize> = vars.iter().enumerate().map(|(i, v)| (v, i)).collect();
    let n = vars.len();
    let mut aug: Vec<Vec<f64>> = Vec::new();
    for (coeffs, b) in &eqs {
        let mut row = vec![0.0; n + 1];
        for (v, c) in coeffs {
            row[idx[v]] += c;
        }
        row[n] = *b;
        aug.push(row);
    }

    let m = aug.len();
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut pr = 0;
    for col in 0..n {
        let mut sel = None;
        for r in pr..m {
            if aug[r][col].abs() > 1e-9 {
                sel = Some(r);
                break;
            }
        }
        let sel = match sel {
            Some(s) => s,
            None => continue,
        };
        aug.swap(pr, sel);
        let pv = aug[pr][col];
        for c in 0..=n {
            aug[pr][c] /= pv;
        }
        for r in 0..m {
            if r != pr && aug[r][col].abs() > 1e-9 {
                let f = aug[r][col];
                for c in 0..=n {
                    aug[r][c] -= f * aug[pr][c];
                }
            }
        }
        pivot_cols.push(col);
        pr += 1;
    }
    for r in 0..m {
        let all_zero = (0..n).all(|c| aug[r][c].abs() < 1e-9);
        if all_zero && aug[r][n].abs() > 1e-9 {
            return None;
        }
    }
    if pivot_cols.len() < n {
        return None;
    }
    let mut sol: Vec<(String, i64)> = pivot_cols
        .iter()
        .enumerate()
        .map(|(i, &col)| (vars[col].clone(), aug[i][n].round() as i64))
        .collect();
    sol.sort_by(|a, b| a.0.cmp(&b.0));
    Some(sol)
}

fn main() {
    let text = "y = x + 1\n5 = x + 3\n10 = z + y + 2";
    match solve(text) {
        None => println!("null"),
        Some(sol) => {
            let body: Vec<String> = sol.iter().map(|(k, v)| format!("  {}: {}", k, v)).collect();
            println!("{{\n{}\n}}", body.join(",\n"));
        }
    }
}
