// Word search L-to-R / U-to-D only: scan each row and column for target substring.
// Time O(R*C*L), Space O(max(R,C)).

fn find_word(m: &Vec<Vec<char>>, target: &str) -> bool {
    let r = m.len();
    let c = if r > 0 { m[0].len() } else { 0 };
    for row in m.iter() {
        let s: String = row.iter().collect();
        if s.contains(target) {
            return true;
        }
    }
    for j in 0..c {
        let col: String = (0..r).map(|i| m[i][j]).collect();
        if col.contains(target) {
            return true;
        }
    }
    false
}

fn main() {
    let matrix = vec![
        vec!['F', 'A', 'C', 'I'],
        vec!['O', 'B', 'Q', 'P'],
        vec!['A', 'N', 'O', 'B'],
        vec!['M', 'A', 'S', 'S'],
    ];
    println!("{}", find_word(&matrix, "FOAM"));
}
