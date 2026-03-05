// Level-order (BFS) traversal of a binary tree using a queue. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	Val   int
	Left  *Node
	Right *Node
}

func levelOrder(root *Node) []int {
	res := []int{}
	if root == nil {
		return res
	}
	queue := []*Node{root}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		res = append(res, cur.Val)
		if cur.Left != nil {
			queue = append(queue, cur.Left)
		}
		if cur.Right != nil {
			queue = append(queue, cur.Right)
		}
	}
	return res
}

func main() {
	root := &Node{Val: 1}
	root.Left = &Node{Val: 2}
	root.Right = &Node{Val: 3}
	root.Right.Left = &Node{Val: 4}
	root.Right.Right = &Node{Val: 5}

	vals := levelOrder(root)
	strs := make([]string, len(vals))
	for i, v := range vals {
		strs[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(strs, ", "))
}
