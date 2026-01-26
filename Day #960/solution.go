// Day 960: jump game - can we reach the last index? Greedy furthest-reach.
// Time O(n), Space O(1).
package main

import "fmt"

func canReach(a []int) bool {
	reach := 0
	for i := 0; i < len(a); i++ {
		if i > reach {
			return false
		}
		if i+a[i] > reach {
			reach = i + a[i]
		}
	}
	return true
}

func main() {
	fmt.Println(canReach([]int{1, 3, 1, 2, 0, 1})) // true
	fmt.Println(canReach([]int{1, 2, 1, 0, 0}))    // false
}
