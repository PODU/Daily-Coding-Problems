// Next higher integer with same number of set bits (snoob bit-twiddle).
// O(1) time, O(1) space.
package main

import "fmt"

func nextSameBits(n uint) uint {
	smallest := n & (^n + 1) // n & -n
	ripple := n + smallest
	ones := n ^ ripple
	ones = (ones >> 2) / smallest
	return ripple | ones
}

func main() {
	fmt.Println(nextSameBits(6)) // 9
}
