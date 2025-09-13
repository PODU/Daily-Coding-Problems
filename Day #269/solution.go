// Day 269: Push dominoes simulation via force/two-pointer scan.
// Left-to-right add +force from R, right-to-left add -force from L, sign decides. Time O(n), Space O(n).
package main

import "fmt"

func pushDominoes(s string) string {
	n := len(s)
	force := make([]int, n)
	f := 0
	for i := 0; i < n; i++ { // rightward push
		switch s[i] {
		case 'R':
			f = n
		case 'L':
			f = 0
		default:
			if f > 0 {
				f--
			}
		}
		force[i] += f
	}
	f = 0
	for i := n - 1; i >= 0; i-- { // leftward push
		switch s[i] {
		case 'L':
			f = n
		case 'R':
			f = 0
		default:
			if f > 0 {
				f--
			}
		}
		force[i] -= f
	}
	res := make([]byte, n)
	for i, v := range force {
		switch {
		case v > 0:
			res[i] = 'R'
		case v < 0:
			res[i] = 'L'
		default:
			res[i] = '.'
		}
	}
	return string(res)
}

func main() {
	fmt.Println(pushDominoes(".L.R....L"))
	fmt.Println(pushDominoes("..R...L.L"))
}
