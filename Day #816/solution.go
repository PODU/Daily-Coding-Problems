// Branchless select via bitwise mask: y ^ ((-b) & (x ^ y)). O(1) time, O(1) space.
package main

import "fmt"

func selectVal(x, y, b int32) int32 {
	return y ^ ((-b) & (x ^ y))
}

func main() {
	fmt.Printf("b=1 -> %d\n", selectVal(5, 9, 1))
	fmt.Printf("b=0 -> %d\n", selectVal(5, 9, 0))
}
