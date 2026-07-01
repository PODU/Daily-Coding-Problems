// Celebrity finder: two-phase candidate elimination then verify. O(N) knows() calls, O(1) space.
package main

import "fmt"

var knowsMat = [4][4]int{
	{0, 1, 1, 0},
	{0, 0, 1, 0},
	{0, 0, 0, 0},
	{0, 1, 1, 0},
}

func knows(a, b int) bool { return knowsMat[a][b] == 1 }

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
	fmt.Println(findCelebrity(4))
}
