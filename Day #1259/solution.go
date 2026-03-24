// Min flips so all x before all y: single-pass DP. flips=min(flips+1, yCount) on 'x', yCount++ on 'y'. O(n) time, O(1) space.
package main

import "fmt"

func minFlips(s string) int {
	flips, yCount := 0, 0
	for _, c := range s {
		if c == 'y' {
			yCount++
		} else {
			if flips+1 < yCount {
				flips = flips + 1
			} else {
				flips = yCount
			}
		}
	}
	return flips
}

func main() {
	fmt.Println(minFlips("xyxxxyxyy"))
}
