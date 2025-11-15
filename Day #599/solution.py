# Day 599: Ghost game - find starting letters that guarantee player 1 a win.
# Approach: build a trie, minimax over it (landing on a word loses). Time O(total chars), Space O(trie).


class Trie:
    __slots__ = ('ch', 'is_word')

    def __init__(self):
        self.ch = {}
        self.is_word = False


def insert(root, w):
    node = root
    for c in w:
        node = node.ch.setdefault(c, Trie())
    node.is_word = True


def mover_wins(node):
    # True if the player to move from `node` can force the opponent to lose.
    for c, child in node.ch.items():
        if child.is_word:
            continue                 # completing a word => mover loses
        if not mover_wins(child):    # opponent loses => mover wins
            return True
    return False


def winning_starts(dictionary):
    root = Trie()
    for w in dictionary:
        insert(root, w)
    res = []
    for c in sorted(root.ch):
        child = root.ch[c]
        if not child.is_word and not mover_wins(child):
            res.append(c)
    return res


def main():
    dictionary = ["cat", "calf", "dog", "bear"]
    print(' '.join(winning_starts(dictionary)))


if __name__ == '__main__':
    main()
