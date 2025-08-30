// Day 196: Most frequent subtree sum.
// Postorder DFS computing each node's subtree sum, count frequencies in a map.
// Time: O(n), Space: O(n).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func dfs(n *Node, freq map[int]int) int {
	if n == nil {
		return 0
	}
	s := n.Val + dfs(n.Left, freq) + dfs(n.Right, freq)
	freq[s]++
	return s
}

func mostFrequentSubtreeSum(root *Node) int {
	freq := map[int]int{}
	dfs(root, freq)
	best, bestCount := 0, -1
	for k, c := range freq {
		if c > bestCount {
			bestCount = c
			best = k
		}
	}
	return best
}

func main() {
	root := &Node{Val: 5, Left: &Node{Val: 2}, Right: &Node{Val: -5}}
	fmt.Println(mostFrequentSubtreeSum(root)) // 2
}
