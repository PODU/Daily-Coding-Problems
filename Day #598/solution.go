// Day 598: Expected number of rounds flipping n coins (remove tails) until one remains.
// DP: f(n) = (1 + 2^-n * sum_{k<n} C(n,k) f(k)) / (1 - 2^-n). Time O(n^2), Space O(n^2).
package main

import (
	"fmt"
	"math"
)

func expectedRounds(n int) float64 {
	if n <= 1 {
		return 0.0
	}
	C := make([][]float64, n+1)
	for i := 0; i <= n; i++ {
		C[i] = make([]float64, n+1)
		C[i][0] = 1.0
		for j := 1; j <= i; j++ {
			C[i][j] = C[i-1][j-1] + C[i-1][j]
		}
	}
	f := make([]float64, n+1)
	for m := 2; m <= n; m++ {
		half := math.Pow(0.5, float64(m))
		s := 0.0
		for k := 0; k < m; k++ {
			s += C[m][k] * f[k]
		}
		f[m] = (1.0 + half*s) / (1.0 - half)
	}
	return f[n]
}

func main() {
	n := 4
	fmt.Printf("%.4f\n", expectedRounds(n)) // ~2.0571
}
