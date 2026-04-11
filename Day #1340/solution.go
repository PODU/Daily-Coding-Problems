// Count grid paths (right/down only) via combinatorics C(N+M-2, M-1).
// Overflow-safe multiplicative loop. Time O(N+M), Space O(1).
package main

import "fmt"

func countPaths(n, m int) uint64 {
	total := n + m - 2
	k := n - 1
	if m-1 < k {
		k = m - 1
	}
	var res uint64 = 1
	for i := 1; i <= k; i++ {
		res = res * uint64(total-k+i) / uint64(i)
	}
	return res
}

func main() {
	fmt.Printf("2 by 2 -> %d\n", countPaths(2, 2))
	fmt.Printf("5 by 5 -> %d\n", countPaths(5, 5))
}
