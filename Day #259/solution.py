# Day 259: Ghost word game. Build a trie of the dictionary. A starting letter is
# guaranteed safe for player 1 iff every word in its subtree has even length, so
# the opponent is always the one forced to complete a word. Trie DFS, O(total chars).


class T:
    def __init__(self):
        self.kids = {}
        self.word = False


def insert(root, w):
    n = root
    for c in w:
        n = n.kids.setdefault(c, T())
    n.word = True


def all_even(n, depth):
    if n.word and depth % 2 != 0:
        return False
    for ch in n.kids.values():
        if not all_even(ch, depth + 1):
            return False
    return True


def winning_starts(words):
    root = T()
    for w in words:
        insert(root, w)
    return [c for c in sorted(root.kids) if all_even(root.kids[c], 1)]


if __name__ == "__main__":
    words = ["cat", "calf", "dog", "bear"]
    print("".join(winning_starts(words)))
