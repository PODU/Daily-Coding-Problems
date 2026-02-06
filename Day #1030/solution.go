// Day 1030: Quxes minimum remaining. Count colors; parity-based O(N) formula.
// If two counts are 0 -> n; else if all parities equal -> 2; else -> 1. Time O(N), Space O(1).
package main

import "fmt"

func minQuxes(q []byte) int {
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
	n := r + g + b
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
	q := []byte{'R', 'G', 'B', 'G', 'B'}
	fmt.Println(minQuxes(q))
}
