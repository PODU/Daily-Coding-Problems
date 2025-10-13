// Day 426: Level of binary tree with minimum sum (levels 0-indexed; root = level 0).
// BFS level-order summing each level, track minimum. Time O(n), Space O(width).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func main() {
	root := &Node{val: 1, left: &Node{val: 2}, right: &Node{val: 3}}

	queue := []*Node{root}
	level, bestLevel := 0, 0
	best := int(^uint(0) >> 1)
	for len(queue) > 0 {
		sz := len(queue)
		s := 0
		for i := 0; i < sz; i++ {
			n := queue[0]
			queue = queue[1:]
			s += n.val
			if n.left != nil {
				queue = append(queue, n.left)
			}
			if n.right != nil {
				queue = append(queue, n.right)
			}
		}
		if s < best {
			best = s
			bestLevel = level
		}
		level++
	}
	fmt.Printf("Level with minimum sum: %d (sum = %d)\n", bestLevel, best)
}
