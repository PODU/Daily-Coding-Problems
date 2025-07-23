// Approach: Single pass, track running path length per depth via a map/stack. O(n) time, O(d) space.
use std::collections::HashMap;

fn length_longest_path(input: &str) -> usize {
    let mut len_at_depth: HashMap<i32, usize> = HashMap::new();
    len_at_depth.insert(-1, 0);
    let mut max_len = 0;
    for line in input.split('\n') {
        let depth = line.chars().take_while(|&c| c == '\t').count();
        let name = &line[depth..];
        let is_file = name.contains('.');
        let cur_len = len_at_depth[&(depth as i32 - 1)] + name.len();
        if is_file {
            max_len = max_len.max(cur_len);
        } else {
            len_at_depth.insert(depth as i32, cur_len + 1); // +1 for '/'
        }
    }
    max_len
}

fn main() {
    let input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    println!("{}", length_longest_path(input));
}
