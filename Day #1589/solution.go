// Quxes minimization: count R/G/B; two zero counts -> n; all parities equal -> 2; else 1.
// Time O(n), Space O(1).
package main

import "fmt"

func minQuxes(a []byte) int {
	r, g, b := 0, 0, 0
	for _, c := range a {
		switch c {
		case 'R':
			r++
		case 'G':
			g++
		default:
			b++
		}
	}
	n := len(a)
	zeros := 0
	if r == 0 {
		zeros++
	}
	if g == 0 {
		zeros++
	}
	if b == 0 {
		zeros++
	}
	if zeros >= 2 {
		return n
	}
	if r%2 == g%2 && g%2 == b%2 {
		return 2
	}
	return 1
}

func main() {
	demo := []byte{'R', 'G', 'B', 'G', 'B'}
	fmt.Println(minQuxes(demo))
}
