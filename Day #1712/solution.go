// In-order traversal -> sorted array, then two-pointer for pair summing to K. Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func inorder(r *Node, acc *[]int) {
	if r == nil {
		return
	}
	inorder(r.Left, acc)
	*acc = append(*acc, r.Val)
	inorder(r.Right, acc)
}

func main() {
	root := &Node{Val: 10}
	root.Left = &Node{Val: 5}
	root.Right = &Node{Val: 15}
	root.Right.Left = &Node{Val: 11}
	root.Right.Right = &Node{Val: 15}
	k := 20
	var a []int
	inorder(root, &a)
	i, j := 0, len(a)-1
	for i < j {
		s := a[i] + a[j]
		if s == k {
			fmt.Printf("%d and %d\n", a[i], a[j])
			return
		} else if s < k {
			i++
		} else {
			j--
		}
	}
	fmt.Println("No pair found")
}
