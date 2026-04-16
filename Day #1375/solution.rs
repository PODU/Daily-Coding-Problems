// Word circle: backtracking to order all words so each last char == next first char,
// and the last wraps to the first. Time O(n!) worst, Space O(n). (n small)
fn last_byte(s: &str) -> u8 {
    s.as_bytes()[s.len() - 1]
}

fn bt(words: &[&str], order: &mut Vec<usize>, used: &mut Vec<bool>) -> bool {
    if order.len() == words.len() {
        return last_byte(words[*order.last().unwrap()]) == words[order[0]].as_bytes()[0];
    }
    let need = last_byte(words[*order.last().unwrap()]);
    for i in 0..words.len() {
        if !used[i] && words[i].as_bytes()[0] == need {
            used[i] = true;
            order.push(i);
            if bt(words, order, used) {
                return true;
            }
            order.pop();
            used[i] = false;
        }
    }
    false
}

fn circle(words: &[&str]) -> Option<Vec<usize>> {
    let mut used = vec![false; words.len()];
    let mut order = vec![0];
    used[0] = true;
    if bt(words, &mut order, &mut used) {
        Some(order)
    } else {
        None
    }
}

fn main() {
    let words = ["chair", "height", "racket", "touch", "tunic"];
    match circle(&words) {
        None => println!("Cannot form a circle"),
        Some(order) => {
            let parts: Vec<&str> = order.iter().map(|&i| words[i]).collect();
            println!("{} --> {}", parts.join(" --> "), words[order[0]]);
        }
    }
}
