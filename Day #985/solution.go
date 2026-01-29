// Count valid Android unlock patterns of length N via backtracking with a
// skip[a][b] over-jump table; use 8-fold symmetry of corners/edges/center.
// Time: O(N * 9!) worst-case bounded by symmetry; Space: O(9).
package main

import "fmt"

var skip [10][10]int
var used [10]bool

func dfs(cur, remaining int) int {
	if remaining == 0 {
		return 1
	}
	used[cur] = true
	count := 0
	for next := 1; next <= 9; next++ {
		if used[next] {
			continue
		}
		mid := skip[cur][next]
		if mid == 0 || used[mid] {
			count += dfs(next, remaining-1)
		}
	}
	used[cur] = false
	return count
}

func countPatterns(n int) int {
	for i := range used {
		used[i] = false
	}
	return 4*dfs(1, n-1) + 4*dfs(2, n-1) + dfs(5, n-1)
}

func main() {
	skip[1][3], skip[3][1] = 2, 2
	skip[1][7], skip[7][1] = 4, 4
	skip[3][9], skip[9][3] = 6, 6
	skip[7][9], skip[9][7] = 8, 8
	skip[1][9], skip[9][1] = 5, 5
	skip[3][7], skip[7][3] = 5, 5
	skip[2][8], skip[8][2] = 5, 5
	skip[4][6], skip[6][4] = 5, 5

	for n := 1; n <= 9; n++ {
		fmt.Printf("N=%d: %d\n", n, countPatterns(n))
	}
}
