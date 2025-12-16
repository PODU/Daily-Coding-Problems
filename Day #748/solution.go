// Most frequent subtree sum: post-order DFS computes each subtree sum, count in a map.
// Time: O(n), Space: O(n).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func dfs(n *Node, count map[int]int) int {
	if n == nil {
		return 0
	}
	s := n.val + dfs(n.left, count) + dfs(n.right, count)
	count[s]++
	return s
}

func mostFrequentSubtreeSum(root *Node) int {
	count := map[int]int{}
	dfs(root, count)
	best, bestCount := 0, -1
	for sum, c := range count {
		if c > bestCount {
			bestCount = c
			best = sum
		}
	}
	return best
}

func main() {
	root := &Node{5, &Node{2, nil, nil}, &Node{-5, nil, nil}}
	fmt.Println(mostFrequentSubtreeSum(root)) // 2
}
