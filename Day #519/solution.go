// Branchless select: mask = -b (all 1s if b=1, all 0s if b=0); pick x or y. O(1).
package main

import "fmt"

func selectXY(x, y, b int32) int32 {
	mask := -b
	return (x & mask) | (y &^ mask)
}

func main() {
	fmt.Println(selectXY(42, 17, 1)) // 42
	fmt.Println(selectXY(42, 17, 0)) // 17
}
