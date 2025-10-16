// Day 442: Cartesian tree (min-heap ordered, in-order == S) built with a
// monotonic stack in O(n) time, O(n) space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func buildCartesian(s []int) *Node {
	var stack []*Node
	for _, v := range s {
		node := &Node{val: v}
		var last *Node
		for len(stack) > 0 && stack[len(stack)-1].val > v {
			last = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		node.left = last
		if len(stack) > 0 {
			stack[len(stack)-1].right = node
		}
		stack = append(stack, node)
	}
	if len(stack) == 0 {
		return nil
	}
	return stack[0]
}

func show(n *Node, prefix, tag string) {
	if n == nil {
		return
	}
	fmt.Println(prefix + tag + fmt.Sprint(n.val))
	show(n.left, prefix+"  ", "L-- ")
	show(n.right, prefix+"  ", "R-- ")
}

func main() {
	root := buildCartesian([]int{3, 2, 6, 1, 9})
	show(root, "", "")
	// 1
	//   L-- 2
	//     L-- 3
	//     R-- 6
	//   R-- 9
}
