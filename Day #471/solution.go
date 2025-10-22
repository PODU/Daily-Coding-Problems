// Generate all structurally distinct BSTs with values 1..N via recursive root selection.
// Time: O(Catalan(N) * N), Space: O(Catalan(N) * N) for the trees.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
}

func generate(start, end int) []*Node {
	if start > end {
		return []*Node{nil}
	}
	var trees []*Node
	for i := start; i <= end; i++ {
		lefts := generate(start, i-1)
		rights := generate(i+1, end)
		for _, l := range lefts {
			for _, r := range rights {
				root := &Node{val: i, left: l, right: r}
				trees = append(trees, root)
			}
		}
	}
	return trees
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
	N := 3
	trees := generate(1, N)
	fmt.Println("Number of BSTs: " + strconv.Itoa(len(trees)))
	for _, t := range trees {
		var out []int
		preorder(t, &out)
		parts := make([]string, len(out))
		for i, v := range out {
			parts[i] = strconv.Itoa(v)
		}
		fmt.Println(strings.Join(parts, " "))
	}
}
