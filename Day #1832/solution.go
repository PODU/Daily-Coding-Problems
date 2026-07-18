// Expected rounds until one coin remains. Each round a coin survives w.p. 1/2.
// DP recurrence: E(n)*(2^n-1) = 2^n + sum_{k=2..n-1} C(n,k) E(k); E(0)=E(1)=0. O(n^2).
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
		C[i][0] = 1
		for j := 1; j <= i; j++ {
			C[i][j] = C[i-1][j-1] + C[i-1][j]
		}
	}
	E := make([]float64, n+1)
	for m := 2; m <= n; m++ {
		pm := math.Pow(2.0, float64(m))
		sum := pm
		for k := 2; k <= m-1; k++ {
			sum += C[m][k] * E[k]
		}
		E[m] = sum / (pm - 1.0)
	}
	return E[n]
}

func main() {
	n := 4
	fmt.Println(expectedRounds(n)) // ~2.05714 rounds
}
