// Jump Game: greedily track the furthest reachable index. O(n) time, O(1) space.
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
	fmt.Println(canJump([]int{1, 3, 1, 2, 0, 1}))
	fmt.Println(canJump([]int{1, 2, 1, 0, 0}))
}
