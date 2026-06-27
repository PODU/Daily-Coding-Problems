// Level of a binary tree with the minimum node-value sum.
// BFS level-order, track the level whose sum is smallest. O(n) time, O(w) space (max width).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

func minSumLevel(root *Node) int {
	if root == nil {
		return -1
	}
	queue := []*Node{root}
	level, bestLevel := 0, 0
	bestSum := math.MaxInt64
	for len(queue) > 0 {
		sum := 0
		var next []*Node
		for _, n := range queue {
			sum += n.val
			if n.left != nil {
				next = append(next, n.left)
			}
			if n.right != nil {
				next = append(next, n.right)
			}
		}
		if sum < bestSum {
			bestSum = sum
			bestLevel = level
		}
		queue = next
		level++
	}
	return bestLevel
}

func main() {
	root := &Node{val: 5,
		left:  &Node{val: 2, left: &Node{val: -5}},
		right: &Node{val: 3}}
	fmt.Println("Level with minimum sum:", minSumLevel(root))
}
