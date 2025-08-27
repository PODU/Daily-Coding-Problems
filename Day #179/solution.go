// Reconstruct BST from postorder: iterate right-to-left as (root,right,left) with a lower bound. O(n) time/space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

var idx int
var post []int

func build(lower int) *Node {
	if idx < 0 || post[idx] < lower {
		return nil
	}
	val := post[idx]
	idx--
	node := &Node{val: val}
	node.right = build(val)
	node.left = build(lower)
	return node
}

func inorder(n *Node, o *[]int) {
	if n != nil {
		inorder(n.left, o)
		*o = append(*o, n.val)
		inorder(n.right, o)
	}
}

func postorder(n *Node, o *[]int) {
	if n != nil {
		postorder(n.left, o)
		postorder(n.right, o)
		*o = append(*o, n.val)
	}
}

func main() {
	post = []int{2, 4, 3, 8, 7, 5}
	idx = len(post) - 1
	root := build(-1 << 31)
	var ino, po []int
	inorder(root, &ino)
	postorder(root, &po)
	fmt.Print("Inorder:")
	for _, x := range ino {
		fmt.Printf(" %d", x)
	}
	fmt.Println()
	fmt.Print("Postorder:")
	for _, x := range po {
		fmt.Printf(" %d", x)
	}
	fmt.Println()
}
