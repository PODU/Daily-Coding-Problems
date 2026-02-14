// Post-order prune: remove subtrees consisting entirely of zeros. Time: O(n), Space: O(n) stack.
package main

import "fmt"

type Node struct {
	val  int
	l, r *Node
}

func prune(n *Node) *Node {
	if n == nil { return nil }
	n.l = prune(n.l)
	n.r = prune(n.r)
	if n.val == 0 && n.l == nil && n.r == nil { return nil }
	return n
}

func preorder(n *Node, out *[]string) {
	if n == nil { *out = append(*out, "X"); return }
	*out = append(*out, fmt.Sprintf("%d", n.val))
	preorder(n.l, out)
	preorder(n.r, out)
}

func main() {
	root := &Node{0,
		&Node{1, nil, nil},
		&Node{0,
			&Node{1, &Node{0, nil, nil}, &Node{0, nil, nil}},
			&Node{0, nil, nil},
		},
	}
	root = prune(root)
	var out []string
	preorder(root, &out)
	fmt.Print("Pruned preorder (X=null):")
	for _, s := range out { fmt.Print(" " + s) }
	fmt.Println()
}
