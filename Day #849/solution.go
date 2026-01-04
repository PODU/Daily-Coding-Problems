// Day 849: Collatz - verify each n reaches 1; bonus: longest chain for n <= 1,000,000.
// Memoized chain lengths via a slice. ~O(limit) amortized.
package main

import "fmt"

func steps(n int64) int {
	c := 0
	for n != 1 {
		if n&1 == 1 {
			n = 3*n + 1
		} else {
			n /= 2
		}
		c++
	}
	return c
}

func main() {
	fmt.Printf("27 reaches 1 in %d steps\n", steps(27)) // 111

	const limit = 1000000
	cache := make([]int, limit+1)
	cache[1] = 1
	bestN, bestLen := 1, 1
	for i := 2; i <= limit; i++ {
		n := int64(i)
		length := 0
		for n >= int64(i) || cache[n] == 0 {
			if n&1 == 1 {
				n = 3*n + 1
			} else {
				n /= 2
			}
			length++
			if n == 1 {
				break
			}
		}
		total := length
		if n <= limit {
			total += cache[n]
		} else {
			total += 1
		}
		cache[i] = total
		if total > bestLen {
			bestLen, bestN = total, i
		}
	}
	fmt.Printf("Longest chain for n <= 1000000: n = %d (length %d)\n", bestN, bestLen) // 837799
}
