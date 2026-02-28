// Bottom view via BFS tracking horizontal distance; last (deepest) node per hd wins. O(n log n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func main() {
	root := &Node{5,
		&Node{3, &Node{1, &Node{0, nil, nil}, nil}, &Node{4, nil, nil}},
		&Node{7, &Node{6, nil, nil}, &Node{9, &Node{8, nil, nil}, nil}}}

	type item struct {
		node *Node
		hd   int
	}
	hdMap := map[int]int{}
	q := []item{{root, 0}}
	for len(q) > 0 {
		it := q[0]
		q = q[1:]
		hdMap[it.hd] = it.node.val
		if it.node.left != nil {
			q = append(q, item{it.node.left, it.hd - 1})
		}
		if it.node.right != nil {
			q = append(q, item{it.node.right, it.hd + 1})
		}
	}
	keys := make([]int, 0, len(hdMap))
	for k := range hdMap {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	parts := make([]string, 0, len(keys))
	for _, k := range keys {
		parts = append(parts, strconv.Itoa(hdMap[k]))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
