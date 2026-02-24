# Day 1122: Day 1122 - Ghost: winning starting letters for player 1 under optimal play
# Approach: build a trie; game-tree minimax over prefixes. A mover loses if every
# letter either completes a word or leads to a position the opponent wins.
# A starting letter wins for player 1 if the next mover (player 2) loses.
# Time: O(total letters), Space: O(total letters).
# Note: for ["cat","calf","dog","bear"] the full winning set is {b, c}
# (player 1 can force a win from c by steering to "calf"); the README spotlights b.

class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_word = False


def build_trie(words):
    root = TrieNode()
    for w in words:
        node = root
        for ch in w:
            node = node.children.setdefault(ch, TrieNode())
        node.is_word = True
    return root


def can_win(node):
    # True if the player about to add a letter to `node` can force a win.
    for ch, child in node.children.items():
        if child.is_word:
            continue                 # moving here spells a word -> mover loses
        if not can_win(child):       # opponent cannot win -> good move
            return True
    return False                     # no safe move -> mover loses


def winning_starts(words):
    root = build_trie(words)
    res = []
    for ch, child in root.children.items():
        if not child.is_word and not can_win(child):
            res.append(ch)
    return sorted(res)


if __name__ == "__main__":
    words = ["cat", "calf", "dog", "bear"]
    print("Winning starting letters:", " ".join(winning_starts(words)))
