// Reconstruct binary tree from preorder+inorder using inorder index hashmap
// and a moving preorder pointer. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val   byte
	left  *Node
	right *Node
}

var idx map[byte]int
var preIdx int
var pre []byte

func build(inL, inR int) *Node {
	if inL > inR {
		return nil
	}
	rootVal := pre[preIdx]
	preIdx++
	root := &Node{val: rootVal}
	mid := idx[rootVal]
	root.left = build(inL, mid-1)
	root.right = build(mid+1, inR)
	return root
}

func preorderT(n *Node, out *[]string) {
	if n == nil {
		return
	}
	*out = append(*out, string(n.val))
	preorderT(n.left, out)
	preorderT(n.right, out)
}
func inorderT(n *Node, out *[]string) {
	if n == nil {
		return
	}
	inorderT(n.left, out)
	*out = append(*out, string(n.val))
	inorderT(n.right, out)
}
func postorderT(n *Node, out *[]string) {
	if n == nil {
		return
	}
	postorderT(n.left, out)
	postorderT(n.right, out)
	*out = append(*out, string(n.val))
}

func main() {
	pre = []byte{'a', 'b', 'd', 'e', 'c', 'f', 'g'}
	in := []byte{'d', 'b', 'e', 'a', 'f', 'c', 'g'}
	idx = make(map[byte]int)
	for i := 0; i < len(in); i++ {
		idx[in[i]] = i
	}
	preIdx = 0
	root := build(0, len(in)-1)

	var po, pr, io []string
	postorderT(root, &po)
	preorderT(root, &pr)
	inorderT(root, &io)
	fmt.Println("postorder: " + strings.Join(po, " "))
	fmt.Println("preorder:  " + strings.Join(pr, " "))
	fmt.Println("inorder:   " + strings.Join(io, " "))
}
