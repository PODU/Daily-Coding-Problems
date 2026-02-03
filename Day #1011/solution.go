// Branchless max: use 64-bit diff to avoid overflow; sign bit via arithmetic
// right shift selects the larger. No if/branch/compare. Time O(1), Space O(1).
package main

import "fmt"

func maxNoBranch(a, b int) int {
	diff := int64(a) - int64(b) // a - b without int32 overflow
	sign := diff >> 63          // -1 if diff<0 else 0
	return a - int(diff&sign)   // a>=b -> a ; a<b -> b
}

func main() {
	fmt.Printf("max(3, 7) = %d\n", maxNoBranch(3, 7))
	fmt.Printf("max(10, -5) = %d\n", maxNoBranch(10, -5))
}
