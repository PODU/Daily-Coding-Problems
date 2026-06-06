// Falling dominoes: two-pass force accumulation (R adds +, L adds -, decay between).
// Sign of net force decides L/R/.; equal force stays '.'. Time O(n), Space O(n).
package main

import "fmt"

func pushDominoes(d string) string {
	n := len(d)
	force := make([]int, n)
	f := 0
	for i := 0; i < n; i++ {
		switch d[i] {
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
	for i := n - 1; i >= 0; i-- {
		switch d[i] {
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
	for i := 0; i < n; i++ {
		if force[i] > 0 {
			res[i] = 'R'
		} else if force[i] < 0 {
			res[i] = 'L'
		} else {
			res[i] = '.'
		}
	}
	return string(res)
}

func main() {
	fmt.Println(pushDominoes(".L.R....L"))
	fmt.Println(pushDominoes("..R...L.L"))
}
