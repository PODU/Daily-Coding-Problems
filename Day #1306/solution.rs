// Longest absolute file path. Parse tab-indented FS, track cumulative path
// length per depth. Time O(n), Space O(depth).
use std::collections::HashMap;

fn length_longest_path(input: &str) -> usize {
    let mut path_len: HashMap<usize, usize> = HashMap::new();
    path_len.insert(0, 0);
    let mut best = 0;
    for line in input.split('\n') {
        let level = line.chars().take_while(|&c| c == '\t').count();
        let name = &line[level..];
        if name.contains('.') {
            best = best.max(path_len[&level] + name.len());
        } else {
            let v = path_len[&level] + name.len() + 1;
            path_len.insert(level + 1, v);
        }
    }
    best
}

fn main() {
    let s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    println!("{}", length_longest_path(s)); // 32
}
