// Day 434: BST floor (largest <= n) and ceiling (smallest >= n).
// Single O(h) walk: node.val < n -> floor candidate (go right); node.val > n -> ceiling
// candidate (go left); equal -> both are n. O(h) time, O(1) space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func insert(root *Node, v int) *Node {
	if root == nil {
		return &Node{val: v}
	}
	if v < root.val {
		root.left = insert(root.left, v)
	} else {
		root.right = insert(root.right, v)
	}
	return root
}

func floorCeil(root *Node, n int) (int, bool, int, bool) {
	var f, c int
	hf, hc := false, false
	cur := root
	for cur != nil {
		if cur.val == n {
			return n, true, n, true
		}
		if cur.val < n {
			f, hf = cur.val, true
			cur = cur.right
		} else {
			c, hc = cur.val, true
			cur = cur.left
		}
	}
	return f, hf, c, hc
}

func main() {
	var root *Node
	for _, v := range []int{8, 4, 12, 2, 6, 10, 14} {
		root = insert(root, v)
	}
	for _, n := range []int{5, 8, 15, 1} {
		f, hf, c, hc := floorCeil(root, n)
		fs := "None"
		if hf {
			fs = fmt.Sprintf("%d", f)
		}
		cs := "None"
		if hc {
			cs = fmt.Sprintf("%d", c)
		}
		fmt.Printf("n=%d: floor=%s, ceiling=%s\n", n, fs, cs)
	}
}
