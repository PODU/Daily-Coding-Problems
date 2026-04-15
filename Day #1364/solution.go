// Topological DP over DAG: dp[node][c] = max count of letter c on a path ending at node.
// Kahn's algorithm detects cycles (return -1 / null). Time O((V+E)*26), Space O(V*26).
package main

import "fmt"

// largestPathValue returns (value, true); ok=false means a cycle exists (null case).
func largestPathValue(colors string, edges [][2]int) (int, bool) {
	n := len(colors)
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

	processed, ans := 0, 0
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		processed++
		cu := int(colors[u] - 'A')
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
	if processed < n {
		return -1, false // cycle
	}
	return ans, true
}

func main() {
	colors := "ABACA"
	edges := [][2]int{{0, 1}, {0, 2}, {2, 3}, {3, 4}}
	res, ok := largestPathValue(colors, edges)
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Println(res)
	}
}
