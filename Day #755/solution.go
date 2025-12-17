// Day 755: Largest value path in a directed graph.
// Topological DP: dp[u][c] = max count of letter c on a path starting at u.
// Cycle -> value is infinite -> null. Time: O((n+m)*26), Space: O(n*26).
package main

import "fmt"

// returns (value, ok); ok=false means infinite (null)
func largestPathValue(letters string, edges [][2]int) (int, bool) {
	n := len(letters)
	adj := make([][]int, n)
	indeg := make([]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		indeg[e[1]]++
	}

	queue := []int{}
	for i := 0; i < n; i++ {
		if indeg[i] == 0 {
			queue = append(queue, i)
		}
	}
	topo := []int{}
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		topo = append(topo, u)
		for _, v := range adj[u] {
			indeg[v]--
			if indeg[v] == 0 {
				queue = append(queue, v)
			}
		}
	}
	if len(topo) < n {
		return 0, false
	}

	dp := make([][26]int, n)
	for i := 0; i < n; i++ {
		dp[i][letters[i]-'A'] = 1
	}
	best := 0
	for i := len(topo) - 1; i >= 0; i-- {
		u := topo[i]
		uc := int(letters[u] - 'A')
		for _, v := range adj[u] {
			for c := 0; c < 26; c++ {
				add := dp[v][c]
				if c == uc {
					add++
				}
				if add > dp[u][c] {
					dp[u][c] = add
				}
			}
		}
		for c := 0; c < 26; c++ {
			if dp[u][c] > best {
				best = dp[u][c]
			}
		}
	}
	return best, true
}

func main() {
	letters := "ABACA"
	edges := [][2]int{{0, 1}, {0, 2}, {2, 3}, {3, 4}}
	v, ok := largestPathValue(letters, edges)
	if ok {
		fmt.Println(v) // 3
	} else {
		fmt.Println("null")
	}
}
