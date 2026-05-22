// Longest absolute path to a file in a tab-indented filesystem string.
// Track cumulative path length per depth; a name with '.' is a file.
// Time O(n), Space O(depth).
use std::collections::HashMap;

fn longest_path(s: &str) -> usize {
    let mut lens: HashMap<usize, usize> = HashMap::new();
    lens.insert(0, 0);
    let mut max_len = 0;
    for line in s.split('\n') {
        let depth = line.chars().take_while(|&c| c == '\t').count();
        let name = &line[depth..];
        if name.contains('.') {
            let base = *lens.get(&depth).unwrap_or(&0);
            max_len = max_len.max(base + name.len());
        } else {
            let base = *lens.get(&depth).unwrap_or(&0);
            lens.insert(depth + 1, base + name.len() + 1); // +1 for '/'
        }
    }
    max_len
}

fn main() {
    let s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    println!("{}", longest_path(s));
}
