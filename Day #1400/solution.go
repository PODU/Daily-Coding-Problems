// Power of four iff: positive, single set bit (n & (n-1))==0, and that bit sits
// at an even position (n & 0x55555555). O(1) time, O(1) space.
package main

import "fmt"

func isPowerOfFour(n uint32) bool {
	return n != 0 && (n&(n-1)) == 0 && (n&0x55555555) != 0
}

func main() {
	for _, n := range []uint32{1, 4, 16, 5, 64, 63} {
		fmt.Printf("%d -> %t\n", n, isPowerOfFour(n))
	}
}
