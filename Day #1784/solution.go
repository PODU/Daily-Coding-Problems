// Morris in-order traversal: thread tree via predecessor links for O(1) space.
// Time O(N), Space O(1) (excluding output).
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

func morrisInorder(root *Node) {
	var out []string
	cur := root
	for cur != nil {
		if cur.left == nil {
			out = append(out, strconv.Itoa(cur.val))
			cur = cur.right
		} else {
			pre := cur.left
			for pre.right != nil && pre.right != cur {
				pre = pre.right
			}
			if pre.right == nil {
				pre.right = cur
				cur = cur.left
			} else {
				pre.right = nil
				out = append(out, strconv.Itoa(cur.val))
				cur = cur.right
			}
		}
	}
	fmt.Println(strings.Join(out, " "))
}

func main() {
	root := &Node{val: 4}
	root.left = &Node{val: 2}
	root.right = &Node{val: 5}
	root.left.left = &Node{val: 1}
	root.left.right = &Node{val: 3}
	morrisInorder(root)
}
