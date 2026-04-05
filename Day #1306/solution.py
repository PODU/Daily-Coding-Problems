# Day 1306: Longest absolute file path. Parse tab-indented FS, track cumulative path
# length per depth. Time O(n), Space O(depth).

def length_longest_path(input_str: str) -> int:
    path_len = {0: 0}
    best = 0
    for line in input_str.split("\n"):
        level = len(line) - len(line.lstrip("\t"))
        name = line[level:]
        if "." in name:
            best = max(best, path_len[level] + len(name))
        else:
            path_len[level + 1] = path_len[level] + len(name) + 1
    return best


if __name__ == "__main__":
    s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    print(length_longest_path(s))  # 32
