// Min flips so all x's precede all y's. Greedy: res=min(res+1, yCount).
// Time O(n), Space O(1).
package main

import "fmt"

func minFlips(s string) int {
	res, yCount := 0, 0
	for _, ch := range s {
		if ch == 'y' {
			yCount++
		} else if res+1 < yCount {
			res = res + 1
		} else {
			res = yCount
		}
	}
	return res
}

func main() {
	fmt.Println(minFlips("xyxxxyxyy"))
}
