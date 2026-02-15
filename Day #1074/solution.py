# Day 1074: LCA with parent pointers: get depths via parent walk, level-up deeper node, advance both until equal. O(h) time O(1) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.left = self.right = self.parent = None

def link(par, child, is_left):
    child.parent = par
    if is_left: par.left = child
    else:        par.right = child
    return child

def depth(n):
    d = 0
    while n.parent: n = n.parent; d += 1
    return d

def lca(a, b):
    da, db = depth(a), depth(b)
    while da > db: a = a.parent; da -= 1
    while db > da: b = b.parent; db -= 1
    while a is not b: a = a.parent; b = b.parent
    return a

n3 = Node(3)
n5 = link(n3, Node(5), True)
n1 = link(n3, Node(1), False)
n6 = link(n5, Node(6), True)
n2 = link(n5, Node(2), False)
link(n1, Node(0), True)
link(n1, Node(8), False)
n7 = link(n2, Node(7), True)
n4 = link(n2, Node(4), False)

print(f"LCA(7,4) = {lca(n7, n4).val}")
print(f"LCA(6,4) = {lca(n6, n4).val}")
print(f"LCA(7,1) = {lca(n7, n1).val}")
