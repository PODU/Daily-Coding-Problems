// Day 1484: Construct all structurally unique BSTs with N nodes (values 1..N).
// For each root i, combine all left BSTs of (lo..i-1) with all right BSTs of
// (i+1..hi). Count is Catalan(N). Time/Space O(Catalan(N) * N).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func build(lo, hi int) []*Node {
	if lo > hi {
		return []*Node{nil}
	}
	var trees []*Node
	for i := lo; i <= hi; i++ {
		for _, l := range build(lo, i-1) {
			for _, r := range build(i+1, hi) {
				trees = append(trees, &Node{i, l, r})
			}
		}
	}
	return trees
}

func preorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	*out = append(*out, n.val)
	preorder(n.left, out)
	preorder(n.right, out)
}

func main() {
	trees := build(1, 3)
	fmt.Println(len(trees)) // 5
	for _, t := range trees {
		var out []int
		preorder(t, &out)
		fmt.Println(out)
	}
}
