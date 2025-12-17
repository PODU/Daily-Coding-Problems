// Day 758: Rotate a list left by k in place using the 3-reversal trick.
// No copy; ~n swaps total (each reversal swaps floor(len/2) pairs).
// Time: O(n), Space: O(1).
package main

import "fmt"

func reverseRange(a []int, i, j int) int {
	swaps := 0
	for i < j {
		a[i], a[j] = a[j], a[i]
		i++
		j--
		swaps++
	}
	return swaps
}

func rotateLeft(a []int, k int) int {
	n := len(a)
	if n == 0 {
		return 0
	}
	k %= n
	swaps := 0
	swaps += reverseRange(a, 0, k-1)
	swaps += reverseRange(a, k, n-1)
	swaps += reverseRange(a, 0, n-1)
	return swaps
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6}
	swaps := rotateLeft(a, 2)
	fmt.Println(a) // [3 4 5 6 1 2]
	fmt.Println("swaps:", swaps)
}
