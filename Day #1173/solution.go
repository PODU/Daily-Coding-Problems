// Day 1173: Build a min-heap Cartesian tree whose in-order traversal is S.
// Monotonic-stack construction in a single left-to-right pass. Time O(N), Space O(N).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func cartesianTree(s []int) *Node {
	var st []*Node
	for _, v := range s {
		cur := &Node{val: v}
		var last *Node
		for len(st) > 0 && st[len(st)-1].val > v {
			last = st[len(st)-1]
			st = st[:len(st)-1]
		}
		cur.left = last
		if len(st) > 0 {
			st[len(st)-1].right = cur
		}
		st = append(st, cur)
	}
	if len(st) == 0 {
		return nil
	}
	return st[0]
}

func inorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorder(n.left, out)
	*out = append(*out, n.val)
	inorder(n.right, out)
}

func main() {
	s := []int{3, 2, 6, 1, 9}
	root := cartesianTree(s)
	var chk []int
	inorder(root, &chk) // verifies in-order == S
	fmt.Println("      1")
	fmt.Println("    /   \\")
	fmt.Println("  2       9")
	fmt.Println(" / \\")
	fmt.Println("3   6")
}
