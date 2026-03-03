// Day 1146: Dominoes - two-pass force accumulation.
// L->R pass adds rightward force, R->L pass adds leftward; sign decides. O(n) time, O(n) space.
package main

import "fmt"

func pushDominoes(s string) string {
	n := len(s)
	forces := make([]int, n)
	force := 0
	for i := 0; i < n; i++ {
		if s[i] == 'R' {
			force = n
		} else if s[i] == 'L' {
			force = 0
		} else if force > 0 {
			force--
		}
		forces[i] += force
	}
	force = 0
	for i := n - 1; i >= 0; i-- {
		if s[i] == 'L' {
			force = n
		} else if s[i] == 'R' {
			force = 0
		} else if force > 0 {
			force--
		}
		forces[i] -= force
	}
	res := make([]byte, n)
	for i, f := range forces {
		if f > 0 {
			res[i] = 'R'
		} else if f < 0 {
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
