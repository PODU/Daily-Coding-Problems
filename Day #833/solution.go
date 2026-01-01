// Celebrity problem: one candidate via elimination, then verify.
// Two-pointer elimination + verification. Time: O(N) knows calls, Space: O(1).
package main

import "fmt"

// Demo knows matrix: N=4, person 2 is the celebrity.
var M = [][]int{
	{0, 1, 1, 0}, // 0 knows 2
	{0, 0, 1, 0}, // 1 knows 2
	{0, 0, 0, 0}, // 2 (celebrity) knows no one
	{0, 1, 1, 0}, // 3 knows 2
}

func knows(a, b int) bool { return M[a][b] == 1 }

func findCelebrity(n int) int {
	cand := 0
	for i := 1; i < n; i++ {
		if knows(cand, i) {
			cand = i
		}
	}
	for i := 0; i < n; i++ {
		if i == cand {
			continue
		}
		if knows(cand, i) || !knows(i, cand) {
			return -1
		}
	}
	return cand
}

func main() {
	fmt.Println(findCelebrity(len(M)))
}
