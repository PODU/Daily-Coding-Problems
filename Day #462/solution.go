// Day 462: Smallest sparse number (no adjacent set bits) >= N.
// Scan bits low->high, lift each "11" pair upward. Time O(log N), Space O(log N).
package main

import "fmt"

func nextSparse(n int64) int64 {
	if n <= 0 {
		return n
	}
	var bits []int
	for t := n; t > 0; t >>= 1 {
		bits = append(bits, int(t&1))
	}
	bits = append(bits, 0, 0)
	size := len(bits)
	lastFinal := 0
	for i := 1; i < size-1; i++ {
		if bits[i] == 1 && bits[i-1] == 1 && bits[i+1] == 0 {
			bits[i+1] = 1
			for j := i; j >= lastFinal; j-- {
				bits[j] = 0
			}
			lastFinal = i + 1
		}
	}
	var ans int64 = 0
	for i := 0; i < size; i++ {
		if bits[i] == 1 {
			ans |= int64(1) << uint(i)
		}
	}
	return ans
}

func main() {
	fmt.Println(nextSparse(22)) // 32
	fmt.Println(nextSparse(21)) // 21
}
