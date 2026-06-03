// BST floor (largest <= x) & ceiling (smallest >= x). Iterative O(h) time, O(1) space.
// Floor: go right recording when val<=x else left. Ceiling: symmetric.
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

// returns (found, value)
func floorBST(root *Node, x int) (bool, int) {
	found, res := false, 0
	for root != nil {
		if root.val == x {
			return true, x
		}
		if root.val < x {
			found, res = true, root.val
			root = root.r
		} else {
			root = root.l
		}
	}
	return found, res
}

func ceilBST(root *Node, x int) (bool, int) {
	found, res := false, 0
	for root != nil {
		if root.val == x {
			return true, x
		}
		if root.val > x {
			found, res = true, root.val
			root = root.l
		} else {
			root = root.r
		}
	}
	return found, res
}

func show(found bool, v int) string {
	if !found {
		return "None"
	}
	return fmt.Sprintf("%d", v)
}

func query(root *Node, x int) {
	ff, fv := floorBST(root, x)
	cf, cv := ceilBST(root, x)
	fmt.Printf("x=%d -> floor %s, ceiling %s\n", x, show(ff, fv), show(cf, cv))
}

func main() {
	var root *Node
	for _, v := range []int{8, 4, 12, 2, 6, 10, 14} {
		root = insert(root, v)
	}
	query(root, 5)  // floor 4, ceiling 6
	query(root, 8)  // floor 8, ceiling 8
	query(root, 15) // floor 14, ceiling None
	query(root, 1)  // floor None, ceiling 2
}
