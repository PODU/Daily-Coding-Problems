// Celebrity problem: O(N) elimination to one candidate, then O(N) verify.
package main

import "fmt"

var matrix [][]int

func knows(a, b int) bool { return matrix[a][b] == 1 }

func findCelebrity(n int) int {
	cand := 0
	for i := 1; i < n; i++ {
		if knows(cand, i) {
			cand = i
		}
	}
	for i := 0; i < n; i++ {
		if i != cand && (knows(cand, i) || !knows(i, cand)) {
			return -1
		}
	}
	return cand
}

func main() {
	matrix = [][]int{{0, 1, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 0}, {0, 1, 1, 0}}
	fmt.Println(findCelebrity(4))
}
