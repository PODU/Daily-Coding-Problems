// Day 135: Minimum root-to-leaf path sum (with path reconstruction).
// DFS over the tree. O(n) time, O(h) recursion space.
package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func minPath(r *Node) (int, []int) {
	if r == nil {
		return math.MaxInt32, nil
	}
	if r.left == nil && r.right == nil {
		return r.val, []int{r.val}
	}
	bestSum := math.MaxInt32
	var bestPath []int
	for _, c := range []*Node{r.left, r.right} {
		if c == nil {
			continue
		}
		s, p := minPath(c)
		if s < bestSum {
			bestSum, bestPath = s, p
		}
	}
	return bestSum + r.val, append([]int{r.val}, bestPath...)
}

func main() {
	root := &Node{val: 10,
		left:  &Node{val: 5, right: &Node{val: 2}},
		right: &Node{val: 5, right: &Node{val: 1, left: &Node{val: -1}}}}
	total, path := minPath(root)
	parts := make([]string, len(path))
	for i, v := range path {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Printf("%d (path [%s])\n", total, strings.Join(parts, ", "))
}
