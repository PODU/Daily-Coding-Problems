// Topological-order DP over DAG: dp[node][c]=max count of letter c on path from node.
// Kahn's algorithm; cycle => "null". Time O((n+m)*26), Space O(n*26).
package main

import (
	"fmt"
	"strconv"
)

func largestPathValue(s string, edges [][2]int) string {
	n := len(s)
	adj := make([][]int, n)
	indeg := make([]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		indeg[e[1]]++
	}
	dp := make([][26]int, n)
	queue := []int{}
	for i := 0; i < n; i++ {
		if indeg[i] == 0 {
			queue = append(queue, i)
		}
	}
	seen, ans := 0, 0
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		seen++
		cu := int(s[u] - 'A')
		dp[u][cu]++
		if dp[u][cu] > ans {
			ans = dp[u][cu]
		}
		for _, v := range adj[u] {
			for c := 0; c < 26; c++ {
				if dp[u][c] > dp[v][c] {
					dp[v][c] = dp[u][c]
				}
			}
			indeg[v]--
			if indeg[v] == 0 {
				queue = append(queue, v)
			}
		}
	}
	if seen < n {
		return "null"
	}
	return strconv.Itoa(ans)
}

func main() {
	fmt.Println(largestPathValue("ABACA", [][2]int{{0, 1}, {0, 2}, {2, 3}, {3, 4}}))
	fmt.Println(largestPathValue("A", [][2]int{{0, 0}}))
}
