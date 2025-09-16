// Day 284: Find all cousins of a node (same level, different parent) via BFS.
// Time O(N), Space O(N).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func cousins(root *Node, target int) []int {
	q := []*Node{root}
	for len(q) > 0 {
		var next []*Node
		var level []int
		var targetParent *Node
		for _, n := range q {
			for _, c := range []*Node{n.left, n.right} {
				if c != nil {
					level = append(level, c.val)
					if c.val == target {
						targetParent = n
					}
					next = append(next, c)
				}
			}
		}
		if targetParent != nil {
			sib := make(map[int]bool)
			if targetParent.left != nil {
				sib[targetParent.left.val] = true
			}
			if targetParent.right != nil {
				sib[targetParent.right.val] = true
			}
			var res []int
			for _, v := range level {
				if !sib[v] {
					res = append(res, v)
				}
			}
			return res
		}
		q = next
	}
	return nil
}

func main() {
	root := &Node{val: 1,
		left:  &Node{val: 2, left: &Node{val: 4}, right: &Node{val: 5}},
		right: &Node{val: 3, right: &Node{val: 6}}}
	fmt.Println(cousins(root, 4)) // [6]
}
