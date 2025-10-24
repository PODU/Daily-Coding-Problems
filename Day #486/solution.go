// Day 486: Celebrity problem.
// Two-pointer elimination: one candidate survives in O(n) knows() calls, then
// verify in O(n). Total O(n) time, O(1) space.
package main

import "fmt"

// mock relation matrix: knowsMatrix[a][b] == 1 means a knows b
var knowsMatrix [][]int

func knows(a, b int) bool { return knowsMatrix[a][b] == 1 }

func findCelebrity(n int) int {
	candidate := 0
	for i := 1; i < n; i++ {
		if knows(candidate, i) {
			candidate = i
		}
	}
	for i := 0; i < n; i++ {
		if i == candidate {
			continue
		}
		if knows(candidate, i) || !knows(i, candidate) {
			return -1
		}
	}
	return candidate
}

func main() {
	// person 2 is the celebrity: knows nobody, everyone knows them
	knowsMatrix = [][]int{
		{0, 1, 1, 0},
		{1, 0, 1, 1},
		{0, 0, 0, 0},
		{1, 1, 1, 0},
	}
	fmt.Println(findCelebrity(4)) // 2
}
