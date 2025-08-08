// Day 85: Select x if b==1 else y using only bit ops. mask = -b (all 1s or all 0s).
// Time O(1), Space O(1).
package main

import "fmt"

func selectVal(x, y, b int32) int32 {
	mask := -b // b=1 -> 0xFFFFFFFF, b=0 -> 0x00000000
	return (x & mask) | (y &^ mask)
}

func main() {
	fmt.Println(selectVal(5, 10, 1)) // 5
	fmt.Println(selectVal(5, 10, 0)) // 10
}
