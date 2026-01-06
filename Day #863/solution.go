// Day 863: generate() returns a finite but arbitrarily large binary tree in O(1).
// Approach: root created in O(1); children expanded lazily with randomized termination
// (each child exists with prob < 0.5 => branching process is finite almost surely).
// generate(): O(1). Materializing whole tree: O(size). Deterministic demo via MINSTD RNG.
package main

import "fmt"

var rngState int64 = 42

func nextRand() float64 {
	rngState = (rngState * 48271) % 2147483647
	return float64(rngState) / 2147483647.0
}

const P = 0.45
const depthCap = 50

type Node struct {
	left, right *Node
	expanded    bool
}

func ensureChildren(n *Node, depth int) {
	if n.expanded {
		return
	}
	n.expanded = true
	if depth >= depthCap {
		return
	}
	if nextRand() < P {
		n.left = &Node{}
	}
	if nextRand() < P {
		n.right = &Node{}
	}
}

func generate() *Node { return &Node{} } // O(1)

func countNodes(n *Node, depth int) int {
	if n == nil {
		return 0
	}
	ensureChildren(n, depth)
	return 1 + countNodes(n.left, depth+1) + countNodes(n.right, depth+1)
}

func main() {
	root := generate()
	fmt.Printf("Generated a finite binary tree with %d nodes (deterministic demo).\n",
		countNodes(root, 0))
}
