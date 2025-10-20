// Day 461: Number of right/down paths in an N x M grid = C(N+M-2, N-1).
// Multiplicative binomial. Time O(min(N,M)), Space O(1).
package main

import "fmt"

func countPaths(n, m int) int64 {
	a := (n - 1) + (m - 1)
	b := n - 1
	if m-1 < b {
		b = m - 1
	}
	var res int64 = 1
	for i := 1; i <= b; i++ {
		res = res * int64(a-b+i) / int64(i)
	}
	return res
}

func main() {
	fmt.Println(countPaths(2, 2)) // 2
	fmt.Println(countPaths(5, 5)) // 70
}
