// Day 490: Bottom view of a binary tree.
// BFS by horizontal distance (root 0, left hd-1, right hd+1); the last node seen in BFS
// order for each hd is the lowest. Output sorted by hd. Time O(n log n), Space O(n).
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
	hdToVal := map[int]int{}
	queue := []item{{root, 0}}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		hdToVal[cur.hd] = cur.node.val
		if cur.node.left != nil {
			queue = append(queue, item{cur.node.left, cur.hd - 1})
		}
		if cur.node.right != nil {
			queue = append(queue, item{cur.node.right, cur.hd + 1})
		}
	}
	hds := make([]int, 0, len(hdToVal))
	for hd := range hdToVal {
		hds = append(hds, hd)
	}
	sort.Ints(hds)
	res := make([]int, 0, len(hds))
	for _, hd := range hds {
		res = append(res, hdToVal[hd])
	}
	return res
}

func main() {
	root := &Node{5,
		&Node{3, &Node{1, &Node{0, nil, nil}, nil}, &Node{4, nil, nil}},
		&Node{7, &Node{6, nil, nil}, &Node{9, &Node{8, nil, nil}, nil}}}
	fmt.Println(bottomView(root)) // [0 1 3 6 8 9]
}
