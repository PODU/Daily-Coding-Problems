// Paths in grid = C(N+M-2, N-1), computed multiplicatively to avoid overflow. Time O(min(N,M)), Space O(1).
package main

import "fmt"

func countPaths(n, m int) int64 {
	total := n + m - 2
	k := n - 1
	if m-1 < k {
		k = m - 1
	}
	var res int64 = 1
	for i := 1; i <= k; i++ {
		res = res * int64(total-k+i) / int64(i)
	}
	return res
}

func main() {
	fmt.Println(countPaths(2, 2))
}
