// Day 1154: Minimum root-to-leaf path sum.
// DFS tracking running sum/path, keep best at leaves. O(n) time, O(h) space.
package main

import (
	"fmt"
	"math"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

var best = math.MaxInt32
var bestPath []int

func dfs(n *Node, path []int, sum int) {
	if n == nil {
		return
	}
	path = append(path, n.val)
	sum += n.val
	if n.left == nil && n.right == nil {
		if sum < best {
			best = sum
			bestPath = append([]int{}, path...)
		}
	} else {
		dfs(n.left, path, sum)
		dfs(n.right, path, sum)
	}
}

func main() {
	root := &Node{val: 10,
		left:  &Node{val: 5, right: &Node{val: 2}},
		right: &Node{val: 5, right: &Node{val: 1, left: &Node{val: -1}}}}
	dfs(root, []int{}, 0)
	var parts []string
	for _, v := range bestPath {
		parts = append(parts, fmt.Sprintf("%d", v))
	}
	fmt.Printf("[%s], which has sum %d\n", strings.Join(parts, ", "), best)
	// [10, 5, 1, -1], which has sum 15
}
