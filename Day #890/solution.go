// Lattice paths in N x M grid = C(N+M-2, N-1), computed iteratively.
// Time: O(min(N,M)), Space: O(1).
package main

import "fmt"

func paths(n, m int64) int64 {
	total := n + m - 2
	k := n - 1
	if m-1 < k {
		k = m - 1
	}
	var res int64 = 1
	for i := int64(1); i <= k; i++ {
		res = res * (total - k + i) / i
	}
	return res
}

func main() {
	fmt.Println(paths(2, 2))
	fmt.Println(paths(5, 5))
}
