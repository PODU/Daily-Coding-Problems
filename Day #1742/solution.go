// Approach: recursive generation of all BSTs; root choice + Cartesian product of left/right.
// Time/Space O(Catalan(N)*N).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
}

func build(lo, hi int) []*Node {
	if lo > hi {
		return []*Node{nil}
	}
	var res []*Node
	for r := lo; r <= hi; r++ {
		lefts := build(lo, r-1)
		rights := build(r+1, hi)
		for _, l := range lefts {
			for _, ri := range rights {
				res = append(res, &Node{val: r, left: l, right: ri})
			}
		}
	}
	return res
}

func preorder(n *Node, out *[]string) {
	if n == nil {
		return
	}
	*out = append(*out, strconv.Itoa(n.val))
	preorder(n.left, out)
	preorder(n.right, out)
}

func main() {
	N := 3
	trees := build(1, N)
	fmt.Println(len(trees))
	for _, t := range trees {
		var out []string
		preorder(t, &out)
		fmt.Println(strings.Join(out, ","))
	}
}
