// Day 1177: Find the element appearing once while all others appear 3 times.
// Track bits seen once (ones) and twice (twos); a third sighting clears both.
// Time O(N), Space O(1).
package main

import "fmt"

func singleNumber(a []int) int {
	ones, twos := 0, 0
	for _, x := range a {
		ones = (ones ^ x) &^ twos
		twos = (twos ^ x) &^ ones
	}
	return ones
}

func main() {
	fmt.Println(singleNumber([]int{6, 1, 3, 3, 3, 6, 6})) // 1
	fmt.Println(singleNumber([]int{13, 19, 13, 13}))       // 19
}
