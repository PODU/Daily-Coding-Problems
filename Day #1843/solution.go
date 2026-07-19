// Day 1843: Max of two numbers with no branching/comparison; uses sign bit of the difference.
// max(a,b) = a - ((a-b) & ((a-b) >> 63)). Time O(1), Space O(1).
package main

import "fmt"

func maxNoBranch(a, b int64) int64 {
	d := a - b
	return a - (d & (d >> 63))
}

func main() {
	fmt.Println(maxNoBranch(5, 9))   // 9
	fmt.Println(maxNoBranch(12, 4))  // 12
	fmt.Println(maxNoBranch(-3, -7)) // -3
}
