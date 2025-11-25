// Longest absolute file path: split on '\n', track pathLen by depth via vec. Time O(n), Space O(depth).
fn length_longest_path(input: &str) -> usize {
    let mut path_len: Vec<usize> = Vec::new();
    let mut max_len = 0;
    for line in input.split('\n') {
        let depth = line.chars().take_while(|&c| c == '\t').count();
        let name = &line[depth..];
        let cur_len = if depth > 0 { path_len[depth - 1] + 1 } else { 0 } + name.len();
        if path_len.len() == depth {
            path_len.push(cur_len);
        } else {
            path_len[depth] = cur_len;
        }
        if name.contains('.') && cur_len > max_len {
            max_len = cur_len;
        }
    }
    max_len
}

fn main() {
    let input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    println!("{}", length_longest_path(input));
}
