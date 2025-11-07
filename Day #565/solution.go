// Jump Game: greedy tracking farthest reachable index. Time O(N), Space O(1).
package main

import "fmt"

func canJump(nums []int) bool {
	farthest := 0
	for i, v := range nums {
		if i > farthest {
			return false
		}
		if i+v > farthest {
			farthest = i + v
		}
	}
	return true
}

func main() {
	if canJump([]int{2, 0, 1, 0}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
	if canJump([]int{1, 1, 0, 1}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
