// Day 943: Count tilings of a 2xN board with 2x1 dominoes and L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. Time O(n), Space O(1).
package main

import "fmt"

func countTilings(n int) int64 {
	if n == 0 {
		return 1
	}
	if n == 1 {
		return 1
	}
	if n == 2 {
		return 2
	}
	var a, b, c int64 = 1, 1, 2
	for i := 3; i <= n; i++ {
		f := 2*c + a
		a, b, c = b, c, f
	}
	return c
}

func main() {
	fmt.Println(countTilings(4)) // 11
}
