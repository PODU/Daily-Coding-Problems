// Day 1856: Collatz conjecture test + longest chain under 1,000,000.
// Memoized step counts (cache values <= LIMIT). ~O(LIMIT * avg-chain) time, O(LIMIT) space.
package main

import "fmt"

func main() {
	const LIMIT = 1000000
	steps := make([]int, LIMIT+1) // steps[n] = steps to reach 1 (0 = unknown; steps[1]=0)
	allReach1 := true
	bestN, bestSteps := 1, 0

	for i := 2; i <= LIMIT; i++ {
		n := int64(i)
		cnt := 0
		for n != 1 && (n > LIMIT || steps[n] == 0) {
			if n%2 == 0 {
				n /= 2
			} else {
				n = 3*n + 1
			}
			cnt++
		}
		total := cnt
		if n != 1 {
			total += steps[n]
		}
		steps[i] = total
		if total > bestSteps {
			bestSteps, bestN = total, i
		}
	}

	fmt.Printf("All sequences for n in [1, %d] reach 1: %t\n", LIMIT, allReach1)
	fmt.Printf("Longest sequence under %d: n = %d with %d terms\n", LIMIT, bestN, bestSteps+1)
	// 837799, 525 terms
}
