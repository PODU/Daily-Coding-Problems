// Day 1268: Select x if b==1 else y using only arithmetic/bit ops (no branches).
// y ^ ((x ^ y) & -b): -b is all-ones when b==1, all-zeros when b==0. O(1).
package main

import "fmt"

func selectBit(x, y, b int32) int32 {
	return y ^ ((x ^ y) & (-b))
}

func main() {
	var x, y int32 = 5, 10
	fmt.Println("b=1 ->", selectBit(x, y, 1)) // 5
	fmt.Println("b=0 ->", selectBit(x, y, 0)) // 10
}
