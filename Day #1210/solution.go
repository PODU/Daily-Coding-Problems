// Day 1210: Floor and ceiling of a target in a BST.
// Single root-to-leaf descent updating best candidates. Time O(h), Space O(1).
package main

import "fmt"

type Node struct {
	val  int
	l, r *Node
}

func insert(root *Node, v int) *Node {
	if root == nil {
		return &Node{val: v}
	}
	if v < root.val {
		root.l = insert(root.l, v)
	} else {
		root.r = insert(root.r, v)
	}
	return root
}

func floorCeil(root *Node, x int) (*int, *int) {
	var fl, ce *int
	for root != nil {
		if root.val == x {
			v := x
			return &v, &v
		}
		if root.val < x {
			v := root.val
			fl = &v
			root = root.r
		} else {
			v := root.val
			ce = &v
			root = root.l
		}
	}
	return fl, ce
}

func show(p *int) string {
	if p == nil {
		return "None"
	}
	return fmt.Sprintf("%d", *p)
}

func main() {
	var root *Node
	for _, v := range []int{8, 4, 12, 2, 6, 10, 14} {
		root = insert(root, v)
	}
	fl, ce := floorCeil(root, 7)
	fmt.Printf("floor=%s ceiling=%s\n", show(fl), show(ce)) // floor=6 ceiling=8
}
