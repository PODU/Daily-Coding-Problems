// Min flips so all x precede all y. DP: at each char, flips = min(yCount, flips+1):
// flipping current 'y'->'x' costs all prior y's; flipping current 'x'->'y' costs flips+1. Time O(n), space O(1).
package main

import "fmt"

func minFlips(s string) int {
	flips, y := 0, 0
	for _, c := range s {
		if c == 'y' {
			y++
		} else {
			if y < flips+1 {
				flips = y
			} else {
				flips = flips + 1
			}
		}
	}
	return flips
}

func main() {
	fmt.Println(minFlips("xyxxxyxyy")) // 2
}
