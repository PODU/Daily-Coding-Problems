// Power of four: N>0 && single set bit (N&(N-1))==0 && bit at even position (N & 0x55555555). O(1).
package main

import "fmt"

func isPowerOfFour(n uint32) bool {
	return n > 0 && (n&(n-1)) == 0 && (n&0x55555555) != 0
}

func main() {
	for _, t := range []uint32{16, 8, 1, 64, 5} {
		fmt.Printf("%d -> %t\n", t, isPowerOfFour(t))
	}
}
