// Jump game: greedy, track furthest reachable index.
// Time O(n), Space O(1). Prints "True"/"False" (capitalized) per spec.
package main

import "fmt"

func canJump(a []int) bool {
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
	a := []int{2, 0, 1, 0}
	if canJump(a) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
