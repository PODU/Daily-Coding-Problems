// Day 312: Tilings of a 2xN board with dominoes & L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3). O(N) time.
package main

import "fmt"

func tilings(n int) int64 {
	if n == 0 {
		return 1
	}
	if n == 1 {
		return 1
	}
	if n == 2 {
		return 2
	}
	f := make([]int64, n+1)
	f[0], f[1], f[2] = 1, 1, 2
	for i := 3; i <= n; i++ {
		f[i] = 2*f[i-1] + f[i-3]
	}
	return f[n]
}

func main() {
	fmt.Println(tilings(4)) // 11
}
