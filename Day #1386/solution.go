// Second largest in BST via reverse in-order (right,node,left), stop at 2nd visited node. O(h) space, O(n) worst time.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func secondLargest(root *Node) int {
	st := []*Node{}
	cur := root
	count := 0
	for cur != nil || len(st) > 0 {
		for cur != nil {
			st = append(st, cur)
			cur = cur.right
		}
		cur = st[len(st)-1]
		st = st[:len(st)-1]
		count++
		if count == 2 {
			return cur.val
		}
		cur = cur.left
	}
	return -1
}

func main() {
	root := &Node{val: 5}
	root.left = &Node{val: 3}
	root.left.left = &Node{val: 2}
	root.left.right = &Node{val: 4}
	root.right = &Node{val: 8}
	root.right.left = &Node{val: 7}
	root.right.right = &Node{val: 9}
	fmt.Println(secondLargest(root))
}
