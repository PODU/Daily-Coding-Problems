// Day 1577: Smallest sparse number (no adjacent 1 bits) >= N.
// Repeatedly fix the lowest pair of adjacent ones by clearing low bits and carrying.
// Time: O((log N)^2); Space: O(1).
package main

import "fmt"

func smallestSparse(n uint64) uint64 {
	for n&(n>>1) != 0 {
		i := 0
		for !(((n>>uint(i))&1) != 0 && ((n>>uint(i+1))&1) != 0) {
			i++
		}
		mask := (uint64(1) << uint(i+2)) - 1
		n = (n &^ mask) + (uint64(1) << uint(i+2))
	}
	return n
}

func main() {
	fmt.Println(smallestSparse(21)) // 21
}
