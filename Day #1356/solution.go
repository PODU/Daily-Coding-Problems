// Zigzag (boustrophedon) level order of a binary tree. BFS per level, reverse alternate levels. O(N) time, O(N) space.
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

func zigzag(root *Node) []int {
	res := []int{}
	if root == nil {
		return res
	}
	queue := []*Node{root}
	leftToRight := true
	for len(queue) > 0 {
		sz := len(queue)
		level := make([]int, sz)
		next := []*Node{}
		for i := 0; i < sz; i++ {
			cur := queue[i]
			idx := i
			if !leftToRight {
				idx = sz - 1 - i
			}
			level[idx] = cur.val
			if cur.left != nil {
				next = append(next, cur.left)
			}
			if cur.right != nil {
				next = append(next, cur.right)
			}
		}
		res = append(res, level...)
		queue = next
		leftToRight = !leftToRight
	}
	return res
}

func main() {
	root := &Node{val: 1}
	root.left = &Node{val: 2}
	root.right = &Node{val: 3}
	root.left.left = &Node{val: 4}
	root.left.right = &Node{val: 5}
	root.right.left = &Node{val: 6}
	root.right.right = &Node{val: 7}

	res := zigzag(root)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
