// Day 307: BST floor (largest <= x) and ceiling (smallest >= x). O(h) per query.
package main

import "fmt"

type Node struct {
	v    int
	l, r *Node
}

func insert(root *Node, v int) *Node {
	if root == nil {
		return &Node{v: v}
	}
	if v < root.v {
		root.l = insert(root.l, v)
	} else {
		root.r = insert(root.r, v)
	}
	return root
}

func floorCeil(root *Node, x int) (*int, *int) {
	var floor, ceil *int
	for root != nil {
		if root.v == x {
			val := root.v
			return &val, &val
		}
		if root.v < x {
			val := root.v
			floor = &val
			root = root.r
		} else {
			val := root.v
			ceil = &val
			root = root.l
		}
	}
	return floor, ceil
}

func str(p *int) string {
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
	fl, ce := floorCeil(root, 5)
	fmt.Printf("Floor: %s, Ceiling: %s\n", str(fl), str(ce)) // Floor: 4, Ceiling: 6
}
