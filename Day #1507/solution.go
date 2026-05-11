// Reconstruct BST from postorder. Process postorder from the right with an
// upper-bound recursion (reverse postorder = root,right,left). Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
}

func bstFromPostorder(post []int) *Node {
	idx := len(post) - 1
	var build func(bound float64) *Node
	build = func(bound float64) *Node {
		if idx < 0 || float64(post[idx]) < bound {
			return nil
		}
		root := &Node{val: post[idx]}
		idx--
		root.right = build(float64(root.val))
		root.left = build(bound)
		return root
	}
	return build(math.Inf(-1))
}

func preorder(root *Node, out *[]int) {
	if root == nil {
		return
	}
	*out = append(*out, root.val)
	preorder(root.left, out)
	preorder(root.right, out)
}

func main() {
	post := []int{2, 4, 3, 8, 7, 5}
	root := bstFromPostorder(post)
	var pre []int
	preorder(root, &pre)
	parts := make([]string, len(pre))
	for i, v := range pre {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, " "))
}
