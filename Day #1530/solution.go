// Zigzag (boustrophedon) level-order traversal: alternate direction each level.
// BFS level by level, reverse odd levels. O(n) time, O(n) space.
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func zigzag(root *Node) []int {
	res := []int{}
	if root == nil {
		return res
	}
	queue := []*Node{root}
	leftToRight := true
	for len(queue) > 0 {
		var next []*Node
		level := []int{}
		for _, n := range queue {
			level = append(level, n.Val)
			if n.Left != nil {
				next = append(next, n.Left)
			}
			if n.Right != nil {
				next = append(next, n.Right)
			}
		}
		if !leftToRight {
			for i, j := 0, len(level)-1; i < j; i, j = i+1, j-1 {
				level[i], level[j] = level[j], level[i]
			}
		}
		res = append(res, level...)
		leftToRight = !leftToRight
		queue = next
	}
	return res
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2, Left: &Node{Val: 4}, Right: &Node{Val: 5}},
		Right: &Node{Val: 3, Left: &Node{Val: 6}, Right: &Node{Val: 7}}}
	fmt.Println(zigzag(root)) // [1 3 2 4 5 6 7]
}
