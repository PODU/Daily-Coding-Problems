// Day 1501: Most frequent subtree sum.
// Approach: post-order DFS, accumulate subtree sums in a map, pick max count.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func dfs(node *Node, freq map[int]int) int {
	if node == nil {
		return 0
	}
	sum := node.Val + dfs(node.Left, freq) + dfs(node.Right, freq)
	freq[sum]++
	return sum
}

func mostFrequentSubtreeSum(root *Node) []int {
	freq := map[int]int{}
	dfs(root, freq)
	best := 0
	for _, c := range freq {
		if c > best {
			best = c
		}
	}
	res := []int{}
	for k, v := range freq {
		if v == best {
			res = append(res, k)
		}
	}
	sort.Ints(res)
	return res
}

func main() {
	root := &Node{Val: 5, Left: &Node{Val: 2}, Right: &Node{Val: -5}}
	res := mostFrequentSubtreeSum(root)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, " "))
}
