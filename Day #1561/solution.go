// BFS level-order traversal of a binary tree using a queue. Time O(n), Space O(n).
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

func main() {
	root := &Node{val: 1}
	root.left = &Node{val: 2}
	root.right = &Node{val: 3}
	root.right.left = &Node{val: 4}
	root.right.right = &Node{val: 5}

	var out []string
	queue := []*Node{root}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		out = append(out, strconv.Itoa(n.val))
		if n.left != nil {
			queue = append(queue, n.left)
		}
		if n.right != nil {
			queue = append(queue, n.right)
		}
	}
	fmt.Println(strings.Join(out, ", "))
}
