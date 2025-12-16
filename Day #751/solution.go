// Day 751: In-order traversal with O(1) extra space via Morris Traversal.
// Time: O(n), Space: O(1) (re-uses null right pointers as temporary threads).
package main

import (
	"fmt"
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
			pre := cur.left
			for pre.right != nil && pre.right != cur {
				pre = pre.right
			}
			if pre.right == nil { // create thread
				pre.right = cur
				cur = cur.left
			} else { // remove thread and visit
				pre.right = nil
				out = append(out, cur.val)
				cur = cur.right
			}
		}
	}
	return out
}

func main() {
	root := &Node{val: 4,
		left:  &Node{val: 2, left: &Node{val: 1}, right: &Node{val: 3}},
		right: &Node{val: 6, left: &Node{val: 5}, right: &Node{val: 7}}}

	res := morrisInorder(root)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
