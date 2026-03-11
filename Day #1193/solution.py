# Day 1193: Ghost word game: trie + game theory. can_win(node)=mover can force win.
# Winning start c: child not a word AND opponent (can_win(child)) cannot win.
# Time O(total chars), Space O(total chars).
# NOTE: README sample shows only "b" but "c" is also winning.

class Node:
    __slots__ = ("ch", "is_word")
    def __init__(self):
        self.ch = {}
        self.is_word = False


def insert(root, w):
    cur = root
    for c in w:
        if c not in cur.ch:
            cur.ch[c] = Node()
        cur = cur.ch[c]
    cur.is_word = True


def can_win(node):
    # can the player about to move from this prefix force a win?
    for child in node.ch.values():
        if child.is_word:
            continue            # completing a word loses
        if not can_win(child):  # opponent loses
            return True
    return False


def main():
    dict_words = ["cat", "calf", "dog", "bear"]
    root = Node()
    for w in dict_words:
        insert(root, w)

    wins = []
    for c, child in root.ch.items():
        if not child.is_word and not can_win(child):
            wins.append(c)
    wins.sort()
    print(" ".join(wins))


if __name__ == "__main__":
    main()
