// Branchless select: mask = -b (all-ones if b=1 else 0); result = (x & mask) | (y & ^mask). O(1) time/space.
package main

import "fmt"

func selectVal(b int32, x, y int32) int32 {
	mask := -b // 0xFFFFFFFF if b=1, 0 if b=0
	return (x & mask) | (y &^ mask)
}

func main() {
	fmt.Println(selectVal(1, 42, 99))
	fmt.Println(selectVal(0, 42, 99))
}
