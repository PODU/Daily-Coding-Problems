// Day 603: Final resting state of pushed dominoes.
// Approach: two force passes (R rightward, L leftward), compare. Time O(n), Space O(n).
package main

import "fmt"

func pushDominoes(s string) string {
	n := len(s)
	fR := make([]int, n)
	fL := make([]int, n)
	f := 0
	for i := 0; i < n; i++ {
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
		fR[i] = f
	}
	f = 0
	for i := n - 1; i >= 0; i-- {
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
		fL[i] = f
	}
	res := make([]byte, n)
	for i := 0; i < n; i++ {
		if fR[i] > fL[i] {
			res[i] = 'R'
		} else if fR[i] < fL[i] {
			res[i] = 'L'
		} else {
			res[i] = '.'
		}
	}
	return string(res)
}

func main() {
	fmt.Println(pushDominoes(".L.R....L")) // LL.RRRLLL
	fmt.Println(pushDominoes("..R...L.L")) // ..RR.LLLL
}
