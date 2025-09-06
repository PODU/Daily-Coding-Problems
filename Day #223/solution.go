// Day 223: In-order traversal with O(1) extra space (Morris traversal).
// Approach: thread each node to its in-order predecessor temporarily, remove thread after visiting.
// Time O(n), Space O(1) (no stack/recursion).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func morrisInorder(root *Node) []int {
	res := []int{}
	cur := root
	for cur != nil {
		if cur.left == nil {
			res = append(res, cur.val)
			cur = cur.right
		} else {
			pred := cur.left
			for pred.right != nil && pred.right != cur {
				pred = pred.right
			}
			if pred.right == nil {
				pred.right = cur // create thread
				cur = cur.left
			} else {
				pred.right = nil // remove thread
				res = append(res, cur.val)
				cur = cur.right
			}
		}
	}
	return res
}

func main() {
	root := &Node{val: 4,
		left:  &Node{val: 2, left: &Node{val: 1}, right: &Node{val: 3}},
		right: &Node{val: 6, left: &Node{val: 5}, right: &Node{val: 7}}}
	fmt.Println(morrisInorder(root)) // [1 2 3 4 5 6 7]
}
