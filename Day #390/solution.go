// Presence bitset: mark each present value, then report unmarked ones.
// Time: O(N), Space: O(N) bits.  (N = 1,000,000)
package main

import (
	"fmt"
	"strings"
)

func findMissing(present []int, N int) []int {
	seen := make([]bool, N+1)
	for _, x := range present {
		seen[x] = true
	}
	missing := []int{}
	for i := 1; i <= N; i++ {
		if !seen[i] {
			missing = append(missing, i)
		}
	}
	return missing
}

func main() {
	const N = 1000000
	present := make([]int, 0, N-1000)
	for i := 1; i <= N; i++ {
		if i%1000 != 0 {
			present = append(present, i)
		}
	}

	missing := findMissing(present, N)
	fmt.Println("Missing count:", len(missing))
	parts := make([]string, 0, 10)
	for i := 0; i < 10 && i < len(missing); i++ {
		parts = append(parts, fmt.Sprintf("%d", missing[i]))
	}
	fmt.Println("First 10 missing:", strings.Join(parts, " "))
	fmt.Println("Time complexity: O(N), Space complexity: O(N) bits (N = 1,000,000)")
}
