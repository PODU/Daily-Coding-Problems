# Day 383: Mark covered indices for every substring occurrence, then wrap maximal runs.
# Time: O(|s| * sum|sub|), Space: O(|s|).

def embolden(s, lst):
    n = len(s)
    bold = [False] * n
    for sub in lst:
        if not sub:
            continue
        start = s.find(sub)
        while start != -1:
            for i in range(start, start + len(sub)):
                bold[i] = True
            start = s.find(sub, start + 1)
    out = []
    for i in range(n):
        if bold[i] and (i == 0 or not bold[i - 1]):
            out.append("<b>")
        out.append(s[i])
        if bold[i] and (i == n - 1 or not bold[i + 1]):
            out.append("</b>")
    return "".join(out)


if __name__ == "__main__":
    print(embolden("abcdefg", ["bc", "ef"]))
    print(embolden("abcdefg", ["bcd", "def"]))
