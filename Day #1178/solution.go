// Day 1178: Collatz sequences. Verify each reaches 1 and find the longest start <= 1e6.
// Memoized chain lengths (each value cached once). Time ~O(LIMIT), Space O(LIMIT).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

const LIMIT = 1000000

var memo [LIMIT + 1]int32

func clen(n int64) int32 {
	if n == 1 {
		return 1
	}
	if n <= LIMIT && memo[n] != 0 {
		return memo[n]
	}
	var nxt int64
	if n%2 == 0 {
		nxt = n / 2
	} else {
		nxt = 3*n + 1
	}
	l := 1 + clen(nxt)
	if n <= LIMIT {
		memo[n] = l
	}
	return l
}

func main() {
	var seq []string
	n := int64(6)
	for {
		seq = append(seq, strconv.FormatInt(n, 10))
		if n == 1 {
			break
		}
		if n%2 == 0 {
			n = n / 2
		} else {
			n = 3*n + 1
		}
	}
	fmt.Println(strings.Join(seq, " -> "))

	bestN, bestLen := 1, int32(1)
	for i := 2; i <= LIMIT; i++ {
		l := clen(int64(i))
		if l > bestLen {
			bestLen = l
			bestN = i
		}
	}
	fmt.Printf("All sequences up to %d reach 1: true\n", LIMIT)
	fmt.Printf("Longest sequence for n <= %d: n = %d (length %d)\n", LIMIT, bestN, bestLen)
}
