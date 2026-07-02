// Day 1750: Jump game — can advance at most arr[i] steps from index i.
// Greedy: track farthest reachable index in one pass.
// Time O(n), Space O(1).
package main

import "fmt"

func canReach(a []int) bool {
	reach := 0
	for i, step := range a {
		if i > reach {
			return false
		}
		if i+step > reach {
			reach = i + step
		}
	}
	return true
}

func main() {
	fmt.Println(canReach([]int{1, 3, 1, 2, 0, 1})) // true
	fmt.Println(canReach([]int{1, 2, 1, 0, 0}))    // false
}
