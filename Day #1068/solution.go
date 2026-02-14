// Dominoes final state via two-pass force summation. Time: O(n), Space: O(n).
package main

import "fmt"

func dominoes(s string) string {
	n := len(s)
	forces := make([]int, n)
	// Left to right: R force propagates rightward
	f := 0
	for i := 0; i < n; i++ {
		if s[i] == 'R' { f = n } else if s[i] == 'L' { f = 0 } else if f > 0 { f-- }
		forces[i] += f
	}
	// Right to left: L force propagates leftward (subtract)
	f = 0
	for i := n - 1; i >= 0; i-- {
		if s[i] == 'L' { f = n } else if s[i] == 'R' { f = 0 } else if f > 0 { f-- }
		forces[i] -= f
	}
	res := make([]byte, n)
	for i, v := range forces {
		if v > 0 { res[i] = 'R' } else if v < 0 { res[i] = 'L' } else { res[i] = '.' }
	}
	return string(res)
}

func main() {
	fmt.Println(dominoes(".L.R....L"))
	fmt.Println(dominoes("..R...L.L"))
}
