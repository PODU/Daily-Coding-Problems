// Day 125: Two nodes in a BST summing to K.
// Inorder traversal -> sorted values, two-pointer. O(n) time, O(n) space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func inorder(r *Node, out *[]int) {
	if r == nil {
		return
	}
	inorder(r.left, out)
	*out = append(*out, r.val)
	inorder(r.right, out)
}

func twoSum(root *Node, k int) (int, int, bool) {
	var v []int
	inorder(root, &v)
	i, j := 0, len(v)-1
	for i < j {
		s := v[i] + v[j]
		if s == k {
			return v[i], v[j], true
		}
		if s < k {
			i++
		} else {
			j--
		}
	}
	return 0, 0, false
}

func main() {
	root := &Node{val: 10,
		left:  &Node{val: 5},
		right: &Node{val: 15, left: &Node{val: 11}, right: &Node{val: 15}}}
	a, b, _ := twoSum(root, 20)
	fmt.Printf("Return the nodes %d and %d.\n", a, b)
}
