// Single number where others appear 3x: ones/twos bit-counting state machine.
// O(N) time, O(1) space. Works for negatives via two's-complement int.
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
	fmt.Println(singleNumber([]int{6, 1, 3, 3, 3, 6, 6}))
}
