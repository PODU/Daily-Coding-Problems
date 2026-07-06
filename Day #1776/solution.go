// Day 1776: Count valid Android unlock patterns of length N.
// DFS with symmetry + jump-over rules. Time O(N!) bounded by 9!, Space O(9).
package main

import "fmt"

var skip [10][10]int
var visited [10]bool

func dfs(cur, remaining int) int {
	if remaining == 0 {
		return 1
	}
	visited[cur] = true
	cnt := 0
	for nxt := 1; nxt <= 9; nxt++ {
		if visited[nxt] {
			continue
		}
		mid := skip[cur][nxt]
		if mid == 0 || visited[mid] {
			cnt += dfs(nxt, remaining-1)
		}
	}
	visited[cur] = false
	return cnt
}

func countPatterns(n int) int {
	return 4*dfs(1, n-1) + 4*dfs(2, n-1) + dfs(5, n-1)
}

func main() {
	pairs := [][3]int{{1, 3, 2}, {1, 7, 4}, {3, 9, 6}, {7, 9, 8},
		{1, 9, 5}, {3, 7, 5}, {2, 8, 5}, {4, 6, 5}}
	for _, p := range pairs {
		skip[p[0]][p[1]] = p[2]
		skip[p[1]][p[0]] = p[2]
	}
	for n := 1; n <= 9; n++ {
		fmt.Printf("N=%d: %d\n", n, countPatterns(n))
	}
}
