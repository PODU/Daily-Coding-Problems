// Day 429: kth row of Pascal's triangle (0-indexed: row 0 = [1]).
// Multiplicative recurrence row[j] = row[j-1]*(k-j+1)/j. Time O(k), Space O(k).
fn main() {
    let k: i64 = 4;
    let mut row = vec![1i64; (k + 1) as usize];
    for j in 1..=k {
        row[j as usize] = row[(j - 1) as usize] * (k - j + 1) / j;
    }
    let parts: Vec<String> = row.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
