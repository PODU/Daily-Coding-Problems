// Day 217: Smallest sparse number (no two adjacent set bits) >= N.
// Approach: scan bits low->high; on adjacent 11 with a 0 above, carry up and clear lower bits.
// Time O(bits) ~ O(log N), much faster than O(N log N). Space O(log N).
package main

import "fmt"

func nextSparse(n int64) int64 {
	if n <= 0 {
		return 0
	}
	bits := []int{}
	x := n
	for x != 0 {
		bits = append(bits, int(x&1))
		x >>= 1
	}
	bits = append(bits, 0, 0)
	lastFinal := 0
	for i := 1; i < len(bits)-1; i++ {
		if bits[i] == 1 && bits[i-1] == 1 && bits[i+1] == 0 {
			bits[i+1] = 1
			for j := i; j >= lastFinal; j-- {
				bits[j] = 0
			}
			lastFinal = i + 1
		}
	}
	var res int64 = 0
	for i := len(bits) - 1; i >= 0; i-- {
		res = (res << 1) | int64(bits[i])
	}
	return res
}

func main() {
	fmt.Println(nextSparse(22)) // -> 32
	fmt.Println(nextSparse(21)) // -> 21
}
