// Min Quxes remaining. One color -> N; all counts same parity -> 2; else 1.
// Time O(n), Space O(1).
package main

import "fmt"

func minQuxes(q string) int {
	if len(q) == 0 {
		return 0
	}
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
	distinct := 0
	for _, v := range []int{r, g, b} {
		if v > 0 {
			distinct++
		}
	}
	if distinct == 1 {
		return len(q)
	}
	if r%2 == g%2 && g%2 == b%2 {
		return 2
	}
	return 1
}

func main() {
	fmt.Println(minQuxes("RGBGB"))
}
