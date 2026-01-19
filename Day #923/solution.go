// Power of four via O(1) bit manipulation.
// power of two: n>0 && (n&(n-1))==0; set bit in even position: (n & 0x55555555)!=0.
// Time O(1), Space O(1).
package main

import "fmt"

func isPowerOfFour(n uint32) bool {
	return n > 0 && (n&(n-1)) == 0 && (n&0x55555555) != 0
}

func main() {
	tests := []uint32{1, 4, 16, 64, 8, 5, 0}
	for _, n := range tests {
		fmt.Printf("%d: %t\n", n, isPowerOfFour(n))
	}
}
