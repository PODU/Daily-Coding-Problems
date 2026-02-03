// Largest path value in a directed graph: DFS topological memo + color cycle detection.
// dp[u][c] = max count of letter c on a path ending at u. Cycle -> null. O((n+m)*26) time, O(n*26) space.
package main

import "fmt"

var adj [][]int
var dp [][]int
var state []int // 0 unvisited, 1 in-stack, 2 done
var colors string

func dfs(u int) bool {
	state[u] = 1
	for _, v := range adj[u] {
		if state[v] == 1 {
			return false // back edge -> cycle
		}
		if state[v] == 0 && !dfs(v) {
			return false
		}
	}
	for _, v := range adj[u] {
		for c := 0; c < 26; c++ {
			if dp[v][c] > dp[u][c] {
				dp[u][c] = dp[v][c]
			}
		}
	}
	dp[u][colors[u]-'A']++
	state[u] = 2
	return true
}

func largestPathValue(cols string, edges [][2]int) (int, bool) {
	n := len(cols)
	colors = cols
	adj = make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
	}
	dp = make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, 26)
	}
	state = make([]int, n)
	for i := 0; i < n; i++ {
		if state[i] == 0 && !dfs(i) {
			return 0, false // cycle
		}
	}
	ans := 0
	for i := 0; i < n; i++ {
		for c := 0; c < 26; c++ {
			if dp[i][c] > ans {
				ans = dp[i][c]
			}
		}
	}
	return ans, true
}

func main() {
	r1, ok1 := largestPathValue("ABACA", [][2]int{{0, 1}, {0, 2}, {2, 3}, {3, 4}})
	if ok1 {
		fmt.Println(r1)
	} else {
		fmt.Println("null")
	}
	r2, ok2 := largestPathValue("A", [][2]int{{0, 0}})
	if ok2 {
		fmt.Println(r2)
	} else {
		fmt.Println("null")
	}
}
