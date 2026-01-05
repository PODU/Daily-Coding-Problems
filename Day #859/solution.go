// Day 859: Bottom view of a binary tree.
// Approach: BFS by horizontal distance; last node seen at each hd wins (lowest).
// Time: O(n log n) for ordering, Space: O(n).
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	val         int
	left, right *Node
}

func bottomView(root *Node) []int {
	hdMap := map[int]int{}
	type item struct {
		n  *Node
		hd int
	}
	q := []item{{root, 0}}
	for len(q) > 0 {
		it := q[0]
		q = q[1:]
		hdMap[it.hd] = it.n.val
		if it.n.left != nil {
			q = append(q, item{it.n.left, it.hd - 1})
		}
		if it.n.right != nil {
			q = append(q, item{it.n.right, it.hd + 1})
		}
	}
	var keys []int
	for k := range hdMap {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	var res []int
	for _, k := range keys {
		res = append(res, hdMap[k])
	}
	return res
}

func main() {
	root := &Node{val: 5,
		left:  &Node{val: 3, left: &Node{val: 1, left: &Node{val: 0}}, right: &Node{val: 4}},
		right: &Node{val: 7, left: &Node{val: 6}, right: &Node{val: 9, left: &Node{val: 8}}}}
	fmt.Println(bottomView(root))
}
