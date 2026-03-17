// Post-order DFS computing subtree sums, count frequencies in a hashmap,
// return sum(s) with max frequency. O(n) time, O(n) space.
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

func mostFrequentSubtreeSum(root *Node) []int {
	freq := map[int]int{}
	var dfs func(*Node) int
	dfs = func(n *Node) int {
		if n == nil {
			return 0
		}
		s := n.Val + dfs(n.Left) + dfs(n.Right)
		freq[s]++
		return s
	}
	dfs(root)
	best := 0
	for _, c := range freq {
		if c > best {
			best = c
		}
	}
	var res []int
	for s, c := range freq {
		if c == best {
			res = append(res, s)
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
