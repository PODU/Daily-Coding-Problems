// Day 808: In-order traversal of a binary tree using O(1) extra space (Morris).
// Thread predecessor's right pointer to current, then unthread. Time O(N), Space O(1).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func morrisInorder(root *Node) []int {
	var out []int
	cur := root
	for cur != nil {
		if cur.left == nil {
			out = append(out, cur.val)
			cur = cur.right
		} else {
			pred := cur.left
			for pred.right != nil && pred.right != cur {
				pred = pred.right
			}
			if pred.right == nil {
				pred.right = cur
				cur = cur.left
			} else {
				pred.right = nil
				out = append(out, cur.val)
				cur = cur.right
			}
		}
	}
	return out
}

func main() {
	root := &Node{val: 4}
	root.left = &Node{val: 2}
	root.right = &Node{val: 6}
	root.left.left = &Node{val: 1}
	root.left.right = &Node{val: 3}
	root.right.left = &Node{val: 5}
	root.right.right = &Node{val: 7}
	fmt.Println(morrisInorder(root)) // [1 2 3 4 5 6 7]
}
