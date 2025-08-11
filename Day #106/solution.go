// Day 106: Jump Game - greedy max-reach. O(n) time, O(1) space.
package main

import "fmt"

func canReach(a []int) bool {
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
	b := map[bool]string{true: "True", false: "False"}
	fmt.Println(b[canReach([]int{2, 0, 1, 0})])
	fmt.Println(b[canReach([]int{1, 1, 0, 1})])
}
