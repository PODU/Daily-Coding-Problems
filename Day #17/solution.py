# Day 17: Approach: Single pass, track running path length per depth via a dict/stack. O(n) time, O(d) space.

def length_longest_path(input_str):
    len_at_depth = {-1: 0}
    max_len = 0
    for line in input_str.split("\n"):
        depth = len(line) - len(line.lstrip("\t"))
        name = line[depth:]
        is_file = "." in name
        cur_len = len_at_depth[depth - 1] + len(name)
        if is_file:
            max_len = max(max_len, cur_len)
        else:
            len_at_depth[depth] = cur_len + 1  # +1 for '/'
    return max_len


if __name__ == "__main__":
    s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    print(length_longest_path(s))
