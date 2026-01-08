// Longest absolute file path: track path length per depth in one pass.
// Time O(n), Space O(d) for depth d.
use std::collections::HashMap;

fn length_longest_path(input: &str) -> usize {
    let mut path_len: HashMap<usize, usize> = HashMap::new();
    path_len.insert(0, 0);
    let mut max_len = 0;
    for line in input.split('\n') {
        let depth = line.chars().take_while(|&c| c == '\t').count();
        let name = &line[depth..];
        let prefix = *path_len.get(&depth).unwrap_or(&0);
        if name.contains('.') {
            max_len = max_len.max(prefix + name.len());
        } else {
            path_len.insert(depth + 1, prefix + name.len() + 1);
        }
    }
    max_len
}

fn main() {
    let input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    println!("{}", length_longest_path(input));
}
