// Day 210: Collatz conjecture - verify reach to 1 and find longest sequence for n <= 1e6.
// Memoized sequence lengths (cache for values <= LIMIT). Time: ~O(LIMIT * avg steps), Space: O(LIMIT).
package main

import "fmt"

const LIMIT = 1000000

var cache [LIMIT + 1]int

func collatzLen(start int64) int {
	var path []int64
	m := start
	for m > LIMIT || cache[m] == 0 {
		path = append(path, m)
		if m%2 == 0 {
			m /= 2
		} else {
			m = 3*m + 1
		}
	}
	base := cache[m]
	for i := len(path) - 1; i >= 0; i-- {
		base++
		if path[i] <= LIMIT {
			cache[path[i]] = base
		}
	}
	return base
}

func main() {
	cache[1] = 1
	fmt.Println("Collatz length of 27:", collatzLen(27)) // 112
	bestN, bestLen := 1, 1
	for n := 1; n <= LIMIT; n++ {
		l := collatzLen(int64(n))
		if l > bestLen {
			bestLen, bestN = l, n
		}
	}
	fmt.Printf("Longest sequence for n <= 1000000: n=%d (length %d)\n", bestN, bestLen) // n=837799 (length 525)
}
