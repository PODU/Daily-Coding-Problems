// Day 215: Bottom view of a binary tree.
// Approach: BFS tracking horizontal distance; overwrite map[hd] so last (deepest) node wins. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	val         int
	left, right *Node
}

type item struct {
	node *Node
	hd   int
}

func bottomView(root *Node) []int {
	if root == nil {
		return []int{}
	}
	hdMap := map[int]int{}
	q := []item{{root, 0}}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		hdMap[cur.hd] = cur.node.val
		if cur.node.left != nil {
			q = append(q, item{cur.node.left, cur.hd - 1})
		}
		if cur.node.right != nil {
			q = append(q, item{cur.node.right, cur.hd + 1})
		}
	}
	keys := make([]int, 0, len(hdMap))
	for k := range hdMap {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	res := make([]int, len(keys))
	for i, k := range keys {
		res[i] = hdMap[k]
	}
	return res
}

func main() {
	root := &Node{val: 5,
		left:  &Node{val: 3, left: &Node{val: 1, left: &Node{val: 0}}, right: &Node{val: 4}},
		right: &Node{val: 7, left: &Node{val: 6}, right: &Node{val: 9, left: &Node{val: 8}}}}
	fmt.Println(bottomView(root))
}
