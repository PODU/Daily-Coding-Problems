// Day 117: BFS level by level, track level with minimum sum. O(n) time, O(w) space.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func minSumLevel(root *Node) int {
	if root == nil {
		return -1
	}
	q := []*Node{root}
	level, best := 0, 0
	bestSum := math.MaxInt64
	for len(q) > 0 {
		sum := 0
		var next []*Node
		for _, n := range q {
			sum += n.Val
			if n.Left != nil {
				next = append(next, n.Left)
			}
			if n.Right != nil {
				next = append(next, n.Right)
			}
		}
		if sum < bestSum {
			bestSum = sum
			best = level
		}
		q = next
		level++
	}
	return best
}

func main() {
	root := &Node{Val: 10,
		Left:  &Node{Val: 2, Left: &Node{Val: -5}, Right: &Node{Val: 1}},
		Right: &Node{Val: 3}}
	fmt.Println(minSumLevel(root)) // 2
}
