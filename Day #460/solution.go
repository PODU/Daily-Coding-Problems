// Day 460: Min flips so every 'x' precedes every 'y'.
// One-pass DP: dp = min(flip this x->y, flip all prior y->x). Time O(n), Space O(1).
package main

import "fmt"

func minFlips(s string) int {
	dp, y := 0, 0
	for _, c := range s {
		if c == 'y' {
			y++
		} else if dp+1 < y {
			dp = dp + 1
		} else {
			dp = y
		}
	}
	return dp
}

func main() {
	fmt.Println(minFlips("xyxxxyxyy")) // 2
}
