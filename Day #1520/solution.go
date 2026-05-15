// Single number among triples: bitwise ones/twos accumulators isolate the unique value.
// Time: O(n). Space: O(1).
package main

import "fmt"

func singleNumber(nums []int) int {
	ones, twos := 0, 0
	for _, x := range nums {
		ones = (ones ^ x) &^ twos
		twos = (twos ^ x) &^ ones
	}
	return ones
}

func main() {
	a := []int{6, 1, 3, 3, 3, 6, 6}
	b := []int{13, 19, 13, 13}
	fmt.Println(singleNumber(a)) // 1
	fmt.Println(singleNumber(b)) // 19
}
