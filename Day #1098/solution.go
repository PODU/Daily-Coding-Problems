// Day 1098: Floor and ceiling of x in a BST.
// Walk down the tree updating candidates using BST ordering.
// Time: O(h). Space: O(1).
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

// floorCeil returns (floor, hasFloor, ceil, hasCeil).
func floorCeil(root *Node, x int) (int, bool, int, bool) {
	var fl, ce int
	hf, hc := false, false
	cur := root
	for cur != nil {
		if cur.val == x {
			return x, true, x, true
		}
		if cur.val < x {
			fl, hf = cur.val, true
			cur = cur.r
		} else {
			ce, hc = cur.val, true
			cur = cur.l
		}
	}
	return fl, hf, ce, hc
}

func main() {
	var root *Node
	for _, v := range []int{8, 4, 12, 2, 6, 10, 14} {
		root = insert(root, v)
	}
	fl, hf, ce, hc := floorCeil(root, 5)
	fs, cs := "None", "None"
	if hf {
		fs = fmt.Sprintf("%d", fl)
	}
	if hc {
		cs = fmt.Sprintf("%d", ce)
	}
	fmt.Printf("floor=%s ceil=%s\n", fs, cs) // floor=4 ceil=6
}
