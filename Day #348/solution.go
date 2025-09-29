// Ternary Search Tree with insert/search. Each node: char + left/mid/right + end flag.
// Time: O(L * log A) per op, Space: O(total chars).
package main

import "fmt"

type Node struct {
	c                 byte
	end               bool
	left, mid, right *Node
}

type TST struct{ root *Node }

func (t *TST) insertNode(node *Node, w string, i int) *Node {
	ch := w[i]
	if node == nil {
		node = &Node{c: ch}
	}
	if ch < node.c {
		node.left = t.insertNode(node.left, w, i)
	} else if ch > node.c {
		node.right = t.insertNode(node.right, w, i)
	} else if i+1 < len(w) {
		node.mid = t.insertNode(node.mid, w, i+1)
	} else {
		node.end = true
	}
	return node
}

func (t *TST) insert(w string) {
	if len(w) > 0 {
		t.root = t.insertNode(t.root, w, 0)
	}
}

func (t *TST) search(w string) bool {
	node, i := t.root, 0
	for node != nil {
		ch := w[i]
		if ch < node.c {
			node = node.left
		} else if ch > node.c {
			node = node.right
		} else {
			if i+1 == len(w) {
				return node.end
			}
			i++
			node = node.mid
		}
	}
	return false
}

func main() {
	t := &TST{}
	for _, w := range []string{"code", "cob", "be", "ax", "war", "we"} {
		t.insert(w)
	}
	for _, q := range []string{"code", "cob", "ax", "c", "war", "cat"} {
		fmt.Printf("%s -> %t\n", q, t.search(q))
	}
}
