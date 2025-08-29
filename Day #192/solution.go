// Day 192: Jump game -- can we reach the end (each value caps the jump length).
// Greedy farthest-reach. Time O(n), Space O(1).
package main

import "fmt"

func canReachEnd(a []int) bool {
	reach := 0
	for i, x := range a {
		if i > reach {
			return false
		}
		if i+x > reach {
			reach = i + x
		}
	}
	return true
}

func main() {
	fmt.Println(canReachEnd([]int{1, 3, 1, 2, 0, 1}))
	fmt.Println(canReachEnd([]int{1, 2, 1, 0, 0}))
}
