// Max of two without if/comparison: subtract the masked difference using the sign bit.
// max(a,b) = a - ((a-b) & ((a-b)>>63)). Time O(1), Space O(1).
package main

import "fmt"

func maxNoBranch(a, b int64) int64 {
	diff := a - b
	return a - (diff & (diff >> 63))
}

func main() {
	fmt.Println(maxNoBranch(3, 7))   // 7
	fmt.Println(maxNoBranch(10, -5)) // 10
}
