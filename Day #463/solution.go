// Day 463: Minimum Quxes remaining after merges.
// Approach: if only one color present, none can merge -> total count. Else if all three
// color counts share the same parity -> 2, otherwise -> 1. Time: O(n), Space: O(1).
package main

import "fmt"

func minQuxes(quxes []rune) int {
	r, g, b := 0, 0, 0
	for _, c := range quxes {
		switch c {
		case 'R':
			r++
		case 'G':
			g++
		default:
			b++
		}
	}
	present := 0
	if r > 0 {
		present++
	}
	if g > 0 {
		present++
	}
	if b > 0 {
		present++
	}
	if present <= 1 {
		return r + g + b // all same color (or empty)
	}
	if r%2 == g%2 && g%2 == b%2 {
		return 2
	}
	return 1
}

func main() {
	quxes := []rune{'R', 'G', 'B', 'G', 'B'}
	fmt.Println(minQuxes(quxes))
}
