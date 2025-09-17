// Quxes: adjacent different colors merge to third. Smallest remaining count.
// Count r,g,b; distinct<=1 -> total; all same parity -> 2; else 1. Time O(n), Space O(1).
package main

import "fmt"

func b2i(b bool) int {
	if b {
		return 1
	}
	return 0
}

func smallestQuxes(q []byte) int {
	r, g, b := 0, 0, 0
	for _, c := range q {
		switch c {
		case 'R':
			r++
		case 'G':
			g++
		default:
			b++
		}
	}
	distinct := b2i(r > 0) + b2i(g > 0) + b2i(b > 0)
	if distinct <= 1 {
		return r + g + b
	}
	if r%2 == g%2 && g%2 == b%2 {
		return 2
	}
	return 1
}

func main() {
	q := []byte{'R', 'G', 'B', 'G', 'B'}
	fmt.Println(smallestQuxes(q))
}
