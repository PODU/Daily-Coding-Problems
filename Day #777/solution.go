// Day 777: Ternary Search Tree with insert and search.
// Each node has left/mid/right children. O(L * log A) per op (L=word length).
package main

import "fmt"

type Node struct {
	c       byte
	end     bool
	l, m, r *Node
}

type TST struct{ root *Node }

func insert(node *Node, w string, i int) *Node {
	c := w[i]
	if node == nil {
		node = &Node{c: c}
	}
	if c < node.c {
		node.l = insert(node.l, w, i)
	} else if c > node.c {
		node.r = insert(node.r, w, i)
	} else if i+1 < len(w) {
		node.m = insert(node.m, w, i+1)
	} else {
		node.end = true
	}
	return node
}

func (t *TST) Insert(w string) {
	if w != "" {
		t.root = insert(t.root, w, 0)
	}
}

func (t *TST) Search(w string) bool {
	node, i := t.root, 0
	for node != nil && w != "" {
		c := w[i]
		if c < node.c {
			node = node.l
		} else if c > node.c {
			node = node.r
		} else if i+1 == len(w) {
			return node.end
		} else {
			node = node.m
			i++
		}
	}
	return false
}

func main() {
	t := &TST{}
	for _, w := range []string{"code", "cob", "be", "ax", "war", "we"} {
		t.Insert(w)
	}
	fmt.Println(t.Search("cob"), t.Search("code"), t.Search("cod"), t.Search("we"))
	// true true false true
}
