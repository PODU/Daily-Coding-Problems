// Sorted array -> height-balanced BST: pick lower-middle as root, recurse. Print preorder.
// mid = (lo+hi)/2 (lower-middle). Time O(n), Space O(log n) recursion.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func build(a []int, lo, hi int) *Node {
	if lo > hi {
		return nil
	}
	mid := (lo + hi) / 2
	root := &Node{val: a[mid]}
	root.left = build(a, lo, mid-1)
	root.right = build(a, mid+1, hi)
	return root
}

func preorder(node *Node, out *[]int) {
	if node == nil {
		return
	}
	*out = append(*out, node.val)
	preorder(node.left, out)
	preorder(node.right, out)
}

func main() {
	a := []int{-10, -3, 0, 5, 9}
	root := build(a, 0, len(a)-1)
	var out []int
	preorder(root, &out)
	parts := make([]string, len(out))
	for i, v := range out {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, " "))
}
