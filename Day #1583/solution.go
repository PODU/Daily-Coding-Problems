// Day 1583: Bottom view of a binary tree.
// BFS tracking horizontal distance; last node seen per hd (deepest wins). Time: O(n log n); Space: O(n).
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	val  int
	l, r *Node
}

func bottomView(root *Node) []int {
	if root == nil {
		return []int{}
	}
	hdVal := map[int]int{}
	type item struct {
		n  *Node
		hd int
	}
	q := []item{{root, 0}}
	for len(q) > 0 {
		it := q[0]
		q = q[1:]
		hdVal[it.hd] = it.n.val
		if it.n.l != nil {
			q = append(q, item{it.n.l, it.hd - 1})
		}
		if it.n.r != nil {
			q = append(q, item{it.n.r, it.hd + 1})
		}
	}
	keys := make([]int, 0, len(hdVal))
	for k := range hdVal {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	res := make([]int, 0, len(keys))
	for _, k := range keys {
		res = append(res, hdVal[k])
	}
	return res
}

func main() {
	root := &Node{5,
		&Node{3, &Node{1, &Node{0, nil, nil}, nil}, &Node{4, nil, nil}},
		&Node{7, &Node{6, nil, nil}, &Node{9, &Node{8, nil, nil}, nil}}}
	fmt.Println(bottomView(root)) // [0 1 3 6 8 9]
}
