// Min root-to-leaf path sum via DFS, reconstructing the path. Time O(n), Space O(h).
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

var best int
var bestPath []int

func dfs(n *Node, cur []int) {
	if n == nil {
		return
	}
	cur = append(cur, n.val)
	if n.left == nil && n.right == nil {
		s := 0
		for _, x := range cur {
			s += x
		}
		if s < best {
			best = s
			bestPath = append([]int(nil), cur...)
		}
	} else {
		dfs(n.left, cur)
		dfs(n.right, cur)
	}
}

func main() {
	root := &Node{val: 10}
	root.left = &Node{val: 5}
	root.left.right = &Node{val: 2}
	root.right = &Node{val: 5}
	root.right.right = &Node{val: 1}
	root.right.right.left = &Node{val: -1}

	best = math.MaxInt64
	dfs(root, []int{})

	parts := make([]string, len(bestPath))
	for i, x := range bestPath {
		parts[i] = strconv.Itoa(x)
	}
	fmt.Printf("[%s], which has sum %d.\n", strings.Join(parts, ", "), best)
}
