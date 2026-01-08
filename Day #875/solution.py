# Day 875: Longest absolute file path: track path length per depth in one pass.
# Time O(n), Space O(d) for depth d.

def length_longest_path(s):
    path_len = {0: 0}
    max_len = 0
    for line in s.split("\n"):
        name = line.lstrip("\t")
        depth = len(line) - len(name)
        if "." in name:
            max_len = max(max_len, path_len[depth] + len(name))
        else:
            path_len[depth + 1] = path_len[depth] + len(name) + 1  # +1 for '/'
    return max_len


if __name__ == "__main__":
    s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    print(length_longest_path(s))
