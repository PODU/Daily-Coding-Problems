// Minimum swaps to seat N couples side by side.
// Greedy: partner of p is p^1; swap mismatched partner into place. Time: O(N), Space: O(N).
package main

import "fmt"

func minSwaps(row []int) int {
	n := len(row)
	r := make([]int, n)
	copy(r, row)
	pos := make([]int, n)
	for i, v := range r {
		pos[v] = i
	}
	swaps := 0
	for i := 0; i < n; i += 2 {
		partner := r[i] ^ 1
		if r[i+1] != partner {
			j := pos[partner]
			pos[r[i+1]], pos[r[j]] = j, i+1
			r[i+1], r[j] = r[j], r[i+1]
			swaps++
		}
	}
	return swaps
}

func main() {
	row := []int{0, 2, 1, 3}
	fmt.Println(minSwaps(row))
}
