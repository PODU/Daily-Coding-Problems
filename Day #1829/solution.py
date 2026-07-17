# Day 1829: Ghost: build a trie, solve the game bottom-up. A mover wins if some child is
# not a word and is a losing position for the opponent. O(total chars).
class Trie:
    def __init__(self):
        self.ch = {}
        self.word = False


def insert(root, w):
    cur = root
    for c in w:
        cur = cur.ch.setdefault(c, Trie())
    cur.word = True


def can_win(node):
    for child in node.ch.values():
        if child.word:
            continue  # moving here spells a word => mover loses
        if not can_win(child):
            return True
    return False


if __name__ == "__main__":
    dictionary = ["cat", "calf", "dog", "bear"]
    root = Trie()
    for w in dictionary:
        insert(root, w)

    wins = sorted(c for c, child in root.ch.items()
                  if not child.word and not can_win(child))
    print(", ".join(wins))
