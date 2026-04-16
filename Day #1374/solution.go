// Jump game: greedy track furthest reachable index. Time O(n), Space O(1).
package main

import "fmt"

func canReach(a []int) bool {
	reach := 0
	for i, v := range a {
		if i > reach {
			return false
		}
		if i+v > reach {
			reach = i + v
		}
	}
	return true
}

func main() {
	fmt.Println(canReach([]int{2, 0, 1, 0}))
	fmt.Println(canReach([]int{1, 1, 0, 1}))
}
