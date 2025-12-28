// Monte Carlo pi: sample (x,y) in unit square via deterministic splitmix64 RNG,
// pi ~= 4*inside/total. Fixed seed -> deterministic. Time O(N), Space O(1).
package main

import "fmt"

var state uint64

func nextU64() uint64 {
	state += 0x9E3779B97F4A7C15
	z := state
	z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9
	z = (z ^ (z >> 27)) * 0x94D049BB133111EB
	return z ^ (z >> 31)
}

func nextDouble() float64 {
	return float64(nextU64()>>11) * (1.0 / 9007199254740992.0)
}

func main() {
	state = 42
	const N = 10000000
	inside := 0
	for i := 0; i < N; i++ {
		x := nextDouble()
		y := nextDouble()
		if x*x+y*y <= 1.0 {
			inside++
		}
	}
	pi := 4.0 * float64(inside) / float64(N)
	fmt.Printf("Estimated pi ≈ %.3f\n", pi)
}
