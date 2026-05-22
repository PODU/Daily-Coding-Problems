# Day 1544: Longest absolute path to a file in a tab-indented filesystem string.
# Track cumulative path length per depth; a name with '.' is a file.
# Time O(n), Space O(depth).
def longest_path(s):
    lens = {0: 0}
    max_len = 0
    for line in s.split('\n'):
        name = line.lstrip('\t')
        depth = len(line) - len(name)
        if '.' in name:
            max_len = max(max_len, lens[depth] + len(name))
        else:
            lens[depth + 1] = lens[depth] + len(name) + 1  # +1 for '/'
    return max_len


if __name__ == "__main__":
    s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
    print(longest_path(s))
