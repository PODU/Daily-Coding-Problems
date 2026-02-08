// Cartesian tree (min-heap ordered, in-order = S) via linear stack on right spine.
// Build O(n); rotated-90 print + verification. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func build(s []int) *Node {
	st := []*Node{}
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

func printRotated(n *Node, depth int) {
	if n == nil {
		return
	}
	printRotated(n.right, depth+1)
	fmt.Println(strings.Repeat(" ", depth*4) + fmt.Sprintf("%d", n.val))
	printRotated(n.left, depth+1)
}

func inorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorder(n.left, out)
	*out = append(*out, n.val)
	inorder(n.right, out)
}

func minHeap(n *Node) bool {
	if n == nil {
		return true
	}
	if n.left != nil && n.left.val <= n.val {
		return false
	}
	if n.right != nil && n.right.val <= n.val {
		return false
	}
	return minHeap(n.left) && minHeap(n.right)
}

func main() {
	s := []int{3, 2, 6, 1, 9}
	root := build(s)
	fmt.Printf("Cartesian tree (rotated 90 deg, root=%d):\n", root.val)
	printRotated(root, 0)
	out := []int{}
	inorder(root, &out)
	parts := make([]string, len(out))
	for i, x := range out {
		parts[i] = fmt.Sprintf("%d", x)
	}
	fmt.Println("in-order: " + strings.Join(parts, " "))
	if minHeap(root) {
		fmt.Println("min-heap ordered: true")
	} else {
		fmt.Println("min-heap ordered: false")
	}
}
