// Day 268: Power of four check in O(1).
// Power of two (n & (n-1))==0 AND single bit at even position (n & 0x55555555). Time O(1), Space O(1).
package main

import "fmt"

func isPowerOfFour(n uint32) bool {
	return n != 0 && (n&(n-1)) == 0 && (n&0x55555555) != 0
}

func main() {
	for _, t := range []uint32{16, 8, 64, 5} {
		res := "False"
		if isPowerOfFour(t) {
			res = "True"
		}
		fmt.Printf("%d -> %s\n", t, res)
	}
}
