// XOR all -> a^b; partition by a set bit, XOR each group to recover the two singletons. O(n) time, O(1) space.
package main

import "fmt"

func main() {
	nums := []int{2, 4, 6, 8, 10, 2, 6, 10}
	xorAll := 0
	for _, x := range nums {
		xorAll ^= x
	}
	bit := xorAll & (-xorAll) // lowest set bit
	a, b := 0, 0
	for _, x := range nums {
		if x&bit != 0 {
			a ^= x
		} else {
			b ^= x
		}
	}
	if a > b {
		a, b = b, a
	}
	fmt.Printf("%d and %d\n", a, b)
}
