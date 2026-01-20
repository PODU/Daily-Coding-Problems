// Post-order DFS: compute each subtree sum, tally counts in a map, return most frequent.
// Time O(n), Space O(n).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func dfs(node *Node, counts map[int]int) int {
	if node == nil {
		return 0
	}
	s := node.Val + dfs(node.Left, counts) + dfs(node.Right, counts)
	counts[s]++
	return s
}

func mostFrequentSubtreeSum(root *Node) int {
	counts := make(map[int]int)
	dfs(root, counts)
	best, ans := 0, 0
	first := true
	for k, v := range counts {
		if first || v > best || (v == best && k < ans) {
			best = v
			ans = k
			first = false
		}
	}
	return ans
}

func main() {
	root := &Node{Val: 5, Left: &Node{Val: 2}, Right: &Node{Val: -5}}
	fmt.Println(mostFrequentSubtreeSum(root))
}
