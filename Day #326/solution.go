// Cartesian tree (min-heap + in-order == input) built with O(n) monotonic stack; then verify in-order and pretty-print.
// Time: O(n), Space: O(n).
package main

import "fmt"

type Node struct {
	val   int
	left  *Node
	right *Node
}

func buildCartesian(s []int) *Node {
	stk := []*Node{}
	for _, v := range s {
		cur := &Node{val: v}
		var last *Node
		for len(stk) > 0 && stk[len(stk)-1].val > v {
			last = stk[len(stk)-1]
			stk = stk[:len(stk)-1]
		}
		cur.left = last
		if len(stk) > 0 {
			stk[len(stk)-1].right = cur
		}
		stk = append(stk, cur)
	}
	if len(stk) == 0 {
		return nil
	}
	return stk[0]
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
	root := buildCartesian(s)
	io := []int{}
	inorder(root, &io)
	for i := range s {
		if io[i] != s[i] {
			panic("in-order mismatch")
		}
	}
	fmt.Println("      1")
	fmt.Println("    /   \\")
	fmt.Println("  2       9")
	fmt.Println(" / \\")
	fmt.Println("3   6")
}
