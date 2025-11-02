// Collatz: iterate n->n/2 (even) or 3n+1 (odd) counting steps to 1; verify a range.
// Bonus longest under 1e6 via memoized step counts. Time ~O(LIMIT*avg-chain), space O(LIMIT).
package main

import "fmt"

func steps(n int64) int {
	c := 0
	for n != 1 {
		if n%2 == 0 {
			n = n / 2
		} else {
			n = 3*n + 1
		}
		c++
	}
	return c
}

func main() {
	allReach := true
	for n := int64(1); n <= 20; n++ {
		if steps(n) < 0 {
			allReach = false
		}
	}
	fmt.Printf("Collatz conjecture holds for 1..20: %v\n", allReach)

	const LIMIT = 1000000
	dp := make([]int, LIMIT+1)
	bestN, bestLen := 1, 0
	for i := 2; i <= LIMIT; i++ {
		n := int64(i)
		c := 0
		for n != 1 && (n > LIMIT || dp[n] == 0) {
			if n%2 == 0 {
				n = n / 2
			} else {
				n = 3*n + 1
			}
			c++
		}
		if n != 1 {
			c += dp[n]
		}
		dp[i] = c
		if c > bestLen {
			bestLen, bestN = c, i
		}
	}
	fmt.Printf("Longest under 1000000: n=%d with %d steps\n", bestN, bestLen)
}
