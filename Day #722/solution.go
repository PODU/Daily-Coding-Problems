// Day 722: Minimum swaps so each couple sits side by side.
// Approach: Greedy - fix each even seat; partner = x^1. Each swap places one couple.
// Answer equals N - (#cycles). Time: O(N), Space: O(N).
package main

import "fmt"

func minSwaps(row []int) int {
	r := append([]int(nil), row...)
	pos := make(map[int]int)
	for i, v := range r {
		pos[v] = i
	}
	swaps := 0
	for i := 0; i < len(r); i += 2 {
		partner := r[i] ^ 1
		if r[i+1] != partner {
			j := pos[partner]
			pos[r[i+1]] = j
			pos[partner] = i + 1
			r[i+1], r[j] = r[j], r[i+1]
			swaps++
		}
	}
	return swaps
}

func main() {
	fmt.Println("Minimum swaps:", minSwaps([]int{0, 2, 1, 3})) // 1
}
