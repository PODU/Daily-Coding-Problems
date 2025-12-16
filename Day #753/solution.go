// Day 753: Ternary Search Tree with insert and search.
// Insert/Search: O(L + log n) average, O(L * n) worst; L = key length.
package main

import "fmt"

type tstNode struct {
	c                 byte
	isEnd             bool
	left, mid, right *tstNode
}

type TST struct {
	root *tstNode
}

func (t *TST) insertRec(node *tstNode, s string, i int) *tstNode {
	c := s[i]
	if node == nil {
		node = &tstNode{c: c}
	}
	if c < node.c {
		node.left = t.insertRec(node.left, s, i)
	} else if c > node.c {
		node.right = t.insertRec(node.right, s, i)
	} else if i+1 < len(s) {
		node.mid = t.insertRec(node.mid, s, i+1)
	} else {
		node.isEnd = true
	}
	return node
}

func (t *TST) Insert(s string) {
	if len(s) > 0 {
		t.root = t.insertRec(t.root, s, 0)
	}
}

func (t *TST) Search(s string) bool {
	node, i := t.root, 0
	for node != nil {
		c := s[i]
		if c < node.c {
			node = node.left
		} else if c > node.c {
			node = node.right
		} else {
			if i+1 == len(s) {
				return node.isEnd
			}
			node = node.mid
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
	for _, q := range []string{"code", "cob", "cod", "ax", "hello"} {
		fmt.Printf("%s: %t\n", q, t.Search(q))
	}
}
