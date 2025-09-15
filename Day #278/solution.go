// Day 278: Generate all structurally distinct BSTs with N nodes (values 1..N).
// Recursive divide on root choice. Count = Catalan(N). Time O(Catalan(N)*N).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func build(lo, hi int) []*Node {
	if lo > hi {
		return []*Node{nil}
	}
	var res []*Node
	for r := lo; r <= hi; r++ {
		for _, L := range build(lo, r-1) {
			for _, R := range build(r+1, hi) {
				res = append(res, &Node{val: r, left: L, right: R})
			}
		}
	}
	return res
}

func preorder(n *Node, s *strings.Builder) {
	if n == nil {
		s.WriteString("# ")
		return
	}
	s.WriteString(strconv.Itoa(n.val) + " ")
	preorder(n.left, s)
	preorder(n.right, s)
}

func main() {
	N := 3
	trees := build(1, N)
	fmt.Println("Count:", len(trees)) // 5
	for _, t := range trees {
		var s strings.Builder
		preorder(t, &s)
		fmt.Println(strings.TrimSpace(s.String()))
	}
}
