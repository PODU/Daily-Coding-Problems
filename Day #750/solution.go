// generate() returns a root in O(1); children are materialized lazily on first access.
// Each child exists with probability p<0.5, so the tree is finite (a.s.) yet unbounded.
// generate(): O(1). Traversal materializes nodes on demand.
package main

import (
	"fmt"
	"math/rand"
)

const P = 0.45

type LazyNode struct {
	value       int
	rng         *rand.Rand
	left, right *LazyNode
	lSet, rSet  bool
}

func (n *LazyNode) Left() *LazyNode {
	if !n.lSet {
		n.lSet = true
		if n.rng.Float64() < P {
			n.left = &LazyNode{rng: n.rng}
		}
	}
	return n.left
}

func (n *LazyNode) Right() *LazyNode {
	if !n.rSet {
		n.rSet = true
		if n.rng.Float64() < P {
			n.right = &LazyNode{rng: n.rng}
		}
	}
	return n.right
}

func generate(rng *rand.Rand) *LazyNode { return &LazyNode{rng: rng} } // O(1)

func treeSize(n *LazyNode) int {
	if n == nil {
		return 0
	}
	return 1 + treeSize(n.Left()) + treeSize(n.Right())
}

func main() {
	rng := rand.New(rand.NewSource(42))
	root := generate(rng) // O(1)
	fmt.Println("Generated finite tree size:", treeSize(root))
}
