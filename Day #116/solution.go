// Day 116: generate() returns a root in O(1); children materialize lazily on access.
// Each node spawns children with a depth-decaying probability => finite a.s. but unbounded.
// (Demo uses a seeded Park-Miller LCG so the forced size is reproducible.)
package main

import "fmt"

type Node struct {
	Depth int
	L, R  *Node
}

var lcg int64 = 42

func nextRand() int64 {
	lcg = (lcg * 16807) % 2147483647
	return lcg
}

func threshold(d int) int64 {
	t := int64(750 - 80*d)
	if t > 0 {
		return t
	}
	return 0
}

func generate() *Node { return &Node{Depth: 0} } // O(1)

func force(n *Node) int {
	cnt := 1
	if nextRand()%1000 < threshold(n.Depth) {
		n.L = &Node{Depth: n.Depth + 1}
		cnt += force(n.L)
	}
	if nextRand()%1000 < threshold(n.Depth) {
		n.R = &Node{Depth: n.Depth + 1}
		cnt += force(n.R)
	}
	return cnt
}

func main() {
	root := generate()
	n := force(root)
	fmt.Printf("Generated a finite binary tree with %d nodes\n", n)
}
