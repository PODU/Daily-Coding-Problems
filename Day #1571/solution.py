# Day 1571: Word break reconstruction via DP with backpointers. O(n^2) time, O(n) space.

def word_break(s, words):
    dict_set = set(words)
    n = len(s)
    back = [-2] * (n + 1)  # -2 unreachable, -1 start
    back[0] = -1
    for i in range(1, n + 1):
        for j in range(i):
            if back[j] != -2 and s[j:i] in dict_set:
                back[i] = j
                break
    if back[n] == -2:
        return None
    res = []
    i = n
    while i > 0:
        res.append(s[back[i]:i])
        i = back[i]
    return res[::-1]


if __name__ == "__main__":
    words = ['quick', 'brown', 'the', 'fox']
    s = "thequickbrownfox"
    print(word_break(s, words))
