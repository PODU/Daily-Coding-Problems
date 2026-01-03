// Day 847: jump game - can we reach the last index? Greedy furthest-reach. O(n) time, O(1) space.
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
	fmt.Println(canReach([]int{2, 0, 1, 0})) // true
	fmt.Println(canReach([]int{1, 1, 0, 1})) // false
}
