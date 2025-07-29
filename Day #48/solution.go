// Day 48: Reconstruct binary tree from preorder + inorder.
// Hashmap of inorder positions; recurse. Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         byte
	left, right *Node
}

func buildTree(pre, in []byte) *Node {
	pos := make(map[byte]int)
	for i, v := range in {
		pos[v] = i
	}
	preIdx := 0
	var helper func(inL, inR int) *Node
	helper = func(inL, inR int) *Node {
		if inL > inR {
			return nil
		}
		rootVal := pre[preIdx]
		preIdx++
		root := &Node{val: rootVal}
		mid := pos[rootVal]
		root.left = helper(inL, mid-1)
		root.right = helper(mid+1, inR)
		return root
	}
	return helper(0, len(in)-1)
}

func main() {
	pre := []byte("abdecfg")
	in := []byte("dbeafcg")
	root := buildTree(pre, in)
	// Level-order traversal confirms reconstruction: a b c d e f g
	var out []string
	queue := []*Node{root}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		out = append(out, string(n.val))
		if n.left != nil {
			queue = append(queue, n.left)
		}
		if n.right != nil {
			queue = append(queue, n.right)
		}
	}
	fmt.Println(strings.Join(out, " "))
}
