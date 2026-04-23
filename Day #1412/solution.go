// Day 1412: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: lazy nodes — children are materialized only on access (here via a deterministic
// LCG so the demo is reproducible). generate() itself is O(1); a child appears on first touch.
package main

import "fmt"

var lcg uint64 = 42

func nextRand() int {
	lcg = lcg*1103515245 + 12345
	return int((lcg >> 16) & 0x7fff)
}

type LazyNode struct {
	depth                int
	leftDone, rightDone  bool
	leftCache, rightCache *LazyNode
}

func (n *LazyNode) spawn() bool {
	bound := 55 - n.depth*7
	return bound > 0 && nextRand()%100 < bound
}

func (n *LazyNode) left() *LazyNode {
	if !n.leftDone {
		n.leftDone = true
		if n.spawn() {
			n.leftCache = &LazyNode{depth: n.depth + 1}
		}
	}
	return n.leftCache
}

func (n *LazyNode) right() *LazyNode {
	if !n.rightDone {
		n.rightDone = true
		if n.spawn() {
			n.rightCache = &LazyNode{depth: n.depth + 1}
		}
	}
	return n.rightCache
}

// generate(): O(1)
func generate() *LazyNode { return &LazyNode{depth: 0} }

func countNodes(n *LazyNode) int {
	if n == nil {
		return 0
	}
	l := countNodes(n.left())
	r := countNodes(n.right())
	return 1 + l + r
}

func main() {
	root := generate()
	fmt.Println("Tree size:", countNodes(root))
}
