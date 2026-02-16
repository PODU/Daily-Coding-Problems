// XOR all -> a^b; isolate via lowest set bit; partition & XOR each group to recover a,b; O(n) time O(1) space
package main

import "fmt"

func findTwo(nums []int) (int, int) {
	xorAll := 0
	for _, n := range nums {
		xorAll ^= n
	}
	bit := xorAll & (-xorAll) // lowest set bit that differs between a and b
	a, b := 0, 0
	for _, n := range nums {
		if n&bit != 0 {
			a ^= n
		} else {
			b ^= n
		}
	}
	if a > b {
		a, b = b, a
	}
	return a, b
}

func main() {
	nums := []int{2, 4, 6, 8, 10, 2, 6, 10}
	a, b := findTwo(nums)
	fmt.Printf("%d and %d\n", a, b)
}
