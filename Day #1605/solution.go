// 2xN tiling with dominoes + L-trominoes (LeetCode 790). DP recurrence
// f(n)=2*f(n-1)+f(n-3). O(N) time, O(1) space. Mod 1e9+7 for large N.
package main

import (
	"fmt"
	"strings"
)

const MOD = 1000000007

func numTilings(n int) int64 {
	if n == 0 {
		return 1
	}
	if n == 1 {
		return 1
	}
	if n == 2 {
		return 2
	}
	var a, b, c int64 = 1, 1, 2 // f(0),f(1),f(2)
	for i := 3; i <= n; i++ {
		f := (2*c + a) % MOD
		a, b, c = b, c, f
	}
	return c
}

func main() {
	fmt.Printf("N=4 -> %d\n", numTilings(4)) // 11
	parts := make([]string, 0, 5)
	for n := 1; n <= 5; n++ {
		parts = append(parts, fmt.Sprintf("%d", numTilings(n)))
	}
	fmt.Println("Table N=1..5: " + strings.Join(parts, " ")) // 1 2 5 11 24
}
