# Day 1235: Serialize/deserialize a binary tree via preorder traversal with null markers.
# Time O(n), Space O(n).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def serialize(root):
    out = []

    def go(n):
        if n is None:
            out.append('#')
            return
        # escape to keep token boundaries safe
        out.append(str(n.val).replace('\\', '\\\\').replace('|', '\\|'))
        go(n.left)
        go(n.right)

    go(root)
    return '|'.join(out)


def deserialize(s):
    # split on unescaped '|'
    tokens, cur, i = [], [], 0
    while i < len(s):
        ch = s[i]
        if ch == '\\':
            cur.append(s[i + 1])
            i += 2
        elif ch == '|':
            tokens.append(''.join(cur))
            cur = []
            i += 1
        else:
            cur.append(ch)
            i += 1
    tokens.append(''.join(cur))

    it = iter(tokens)

    def go():
        v = next(it)
        if v == '#':
            return None
        return Node(v, go(), go())

    return go()


if __name__ == "__main__":
    node = Node('root', Node('left', Node('left.left')), Node('right'))
    assert deserialize(serialize(node)).left.left.val == 'left.left'
    print(deserialize(serialize(node)).left.left.val)
