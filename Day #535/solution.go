// Egg drop (min worst-case trials): find smallest m such that with N eggs we can
// cover k floors. f(m, N) = sum_{i=1..N} C(m,i); smallest m with f(m,N) >= k.
// Time: O(m * N) where m is the answer; Space: O(1).
package main

import "fmt"

func eggDrop(n, k int64) int {
	m := 0
	var covered int64 = 0
	for covered < k {
		m++
		var sum int64 = 0
		var term int64 = 1 // term = C(m, i)
		for i := int64(1); i <= n; i++ {
			term = term * (int64(m) - i + 1) / i // C(m,i)
			sum += term
			if sum >= k {
				break
			}
		}
		covered = sum
	}
	return m
}

func main() {
	fmt.Println(eggDrop(1, 5)) // expected 5
}
