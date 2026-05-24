// Lazy binary tree: generate() returns a root in O(1) whose children are thunks
// (func closures) forced on demand; a seeded coin flip (<1 continue prob) makes the
// tree finite w.p.1 but unbounded. Realization is depth-capped for a deterministic demo.
package main

import "fmt"

type LCG struct{ s uint64 }

func (r *LCG) next() uint64 {
	r.s = (r.s * 16807) % 2147483647 // Park-Miller
	return r.s
}
func (r *LCG) coin() bool { return r.next()%100 < 45 } // child exists prob 0.45 -> finite

type Node struct {
	val   int
	left  func() *Node
	right func() *Node
}

var counter int

// makeNode/generate do NOT force children: O(1).
func makeNode(rng *LCG) *Node {
	node := &Node{val: counter}
	counter++
	node.left = func() *Node {
		if rng.coin() {
			return makeNode(rng)
		}
		return nil
	}
	node.right = func() *Node {
		if rng.coin() {
			return makeNode(rng)
		}
		return nil
	}
	return node
}

func generate(rng *LCG) *Node {
	return makeNode(rng) // O(1): one node, children unevaluated
}

func realize(node *Node, depth, cap int) int {
	count := 1
	if depth < cap {
		if l := node.left(); l != nil {
			count += realize(l, depth+1, cap)
		}
		if r := node.right(); r != nil {
			count += realize(r, depth+1, cap)
		}
	}
	return count
}

func main() {
	rng := &LCG{s: 42}
	root := generate(rng) // returns instantly
	fmt.Println("generate() returned a lazy tree root in O(1)")
	n := realize(root, 0, 6)
	fmt.Println("Realized tree node count:", n)
}
