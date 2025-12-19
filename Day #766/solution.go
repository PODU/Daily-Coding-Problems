// Day 766: Minimum flips so all 'x' come before all 'y'.
// One-pass DP: flips = min(flips+1, countY). O(n) time, O(1) space.
package main

import "fmt"

func minFlips(s string) int {
	flips, countY := 0, 0
	for _, c := range s {
		if c == 'y' {
			countY++
		} else if flips+1 < countY {
			flips = flips + 1
		} else {
			flips = countY
		}
	}
	return flips
}

func main() {
	fmt.Println(minFlips("xyxxxyxyy")) // 2
}
