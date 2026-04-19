// BST two-sum: in-order into sorted array, then two-pointer scan for pair summing to K. O(n) time, O(n) space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func inorder(root *Node, a *[]int) {
	if root == nil {
		return
	}
	inorder(root.left, a)
	*a = append(*a, root.val)
	inorder(root.right, a)
}

func twoSum(root *Node, k int) (int, int) {
	a := []int{}
	inorder(root, &a)
	i, j := 0, len(a)-1
	for i < j {
		s := a[i] + a[j]
		if s == k {
			return a[i], a[j]
		}
		if s < k {
			i++
		} else {
			j--
		}
	}
	return -1, -1
}

func main() {
	root := &Node{val: 10}
	root.left = &Node{val: 5}
	root.right = &Node{val: 15}
	root.right.left = &Node{val: 11}
	root.right.right = &Node{val: 15}
	x, y := twoSum(root, 20)
	fmt.Println(x, y)
}
