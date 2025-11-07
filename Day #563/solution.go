// Max of two numbers without if-else/branch/comparison via bit manipulation.
// d=a-b; mask = d>>63 (arithmetic sign extend); max = a - (d & mask). Time O(1), Space O(1).
package main

import "fmt"

func maxNoBranch(a, b int64) int64 {
	d := a - b
	mask := d >> 63 // all 1s if d<0, else 0
	return a - (d & mask)
}

func main() {
	fmt.Println(maxNoBranch(3, 7))
	fmt.Println(maxNoBranch(10, -4))
}
