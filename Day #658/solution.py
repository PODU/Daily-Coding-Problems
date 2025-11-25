# Day 658: Longest absolute file path: split on '\n', track pathLen by depth via dict. Time O(n), Space O(depth).
def length_longest_path(input_str):
    path_len = {0: 0}
    max_len = 0
    for line in input_str.split("\n"):
        depth = len(line) - len(line.lstrip("\t"))
        name = line[depth:]
        cur_len = (path_len[depth] + 1 if depth > 0 else 0) + len(name)
        path_len[depth + 1] = cur_len
        if "." in name:
            max_len = max(max_len, cur_len)
    return max_len

if __name__ == "__main__":
    input_str = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    print(length_longest_path(input_str))
