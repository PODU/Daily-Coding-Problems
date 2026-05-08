// Day 1487: In-order traversal in O(1) space via Morris traversal.
// Time: O(n). Space: O(1) extra.
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

func morrisInorder(root *Node) []int {
	out := []int{}
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
				pred.right = cur // thread
				cur = cur.left
			} else {
				pred.right = nil // restore
				out = append(out, cur.val)
				cur = cur.right
			}
		}
	}
	return out
}

func main() {
	//        4
	//       / \
	//      2   6
	//     / \ / \
	//    1  3 5  7
	root := &Node{val: 4,
		left:  &Node{val: 2, left: &Node{val: 1}, right: &Node{val: 3}},
		right: &Node{val: 6, left: &Node{val: 5}, right: &Node{val: 7}}}

	res := morrisInorder(root)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("In-order:", strings.Join(parts, " "))
}
