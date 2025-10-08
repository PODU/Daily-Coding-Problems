// Largest consecutive range via hash set: from each start (n-1 absent) extend up. O(n) time, O(n) space.
package main

import "fmt"

func largestRange(nums []int) (int, int) {
	s := make(map[int]bool)
	for _, n := range nums {
		s[n] = true
	}
	bestLo, bestHi, bestLen := nums[0], nums[0], 0
	for n := range s {
		if s[n-1] {
			continue
		}
		hi := n
		for s[hi+1] {
			hi++
		}
		if hi-n+1 > bestLen {
			bestLen, bestLo, bestHi = hi-n+1, n, hi
		}
	}
	return bestLo, bestHi
}

func main() {
	nums := []int{9, 6, 1, 3, 8, 10, 12, 11}
	lo, hi := largestRange(nums)
	fmt.Printf("(%d, %d)\n", lo, hi)
}
