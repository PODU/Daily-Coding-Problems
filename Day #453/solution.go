// Day 453: Two nodes in a BST summing to K.
// Inorder -> sorted slice, then two-pointer. Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func inorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	inorder(n.left, out)
	*out = append(*out, n.val)
	inorder(n.right, out)
}

func twoSum(root *Node, k int) (int, int, bool) {
	var a []int
	inorder(root, &a)
	i, j := 0, len(a)-1
	for i < j {
		s := a[i] + a[j]
		if s == k {
			return a[i], a[j], true
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
	x, y, _ := twoSum(root, 20)
	fmt.Printf("%d and %d\n", x, y) // 5 and 15
}
