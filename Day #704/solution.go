// Day 704: Count valid Android unlock patterns of length N on a 3x3 keypad.
// Approach: DFS with a "skip" table (a jump is legal only if the middle key was
// already used); symmetry cuts the work. Time O(N!) worst but tiny (<=9 keys).
package main

import "fmt"

var skip [10][10]int
var visited [10]bool

func dfs(cur, remaining int) int {
	if remaining == 0 {
		return 1
	}
	visited[cur] = true
	count := 0
	for nx := 1; nx <= 9; nx++ {
		if !visited[nx] {
			mid := skip[cur][nx]
			if mid == 0 || visited[mid] {
				count += dfs(nx, remaining-1)
			}
		}
	}
	visited[cur] = false
	return count
}

func numberOfPatterns(n int) int {
	skip[1][3], skip[3][1] = 2, 2
	skip[1][7], skip[7][1] = 4, 4
	skip[3][9], skip[9][3] = 6, 6
	skip[7][9], skip[9][7] = 8, 8
	skip[1][9], skip[9][1], skip[3][7], skip[7][3] = 5, 5, 5, 5
	skip[2][8], skip[8][2], skip[4][6], skip[6][4] = 5, 5, 5, 5
	return 4*dfs(1, n-1) + 4*dfs(2, n-1) + dfs(5, n-1)
}

func main() {
	fmt.Println(numberOfPatterns(4)) // 1624
}
