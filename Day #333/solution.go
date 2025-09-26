// Celebrity problem: 2-pass. Pass 1 eliminate to one candidate; pass 2 verify.
// Time O(n), Space O(1).
package main

import "fmt"

var M = [][]int{
	{0, 1, 1, 0}, // person0 knows {1,2}
	{0, 0, 1, 0}, // person1 knows {2}
	{0, 0, 0, 0}, // person2 knows {}
	{1, 1, 1, 0}, // person3 knows {0,1,2}
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
	fmt.Println(findCelebrity(4))
}
