// Day 1754: All strobogrammatic numbers with N digits.
// Build from middle outward placing rotatable pairs; skip leading 0 for outer layer.
// Time O(output size), space O(N) recursion depth.
package main

import (
	"fmt"
	"strings"
)

var pairs = [][2]byte{{'0', '0'}, {'1', '1'}, {'6', '9'}, {'8', '8'}, {'9', '6'}}

func build(n, total int) []string {
	if n == 0 {
		return []string{""}
	}
	if n == 1 {
		return []string{"0", "1", "8"}
	}
	inner := build(n-2, total)
	var res []string
	for _, s := range inner {
		for _, p := range pairs {
			if p[0] == '0' && n == total { // no leading zero
				continue
			}
			res = append(res, string(p[0])+s+string(p[1]))
		}
	}
	return res
}

func strobogrammatic(n int) []string {
	return build(n, n)
}

func main() {
	for _, n := range []int{2, 3} {
		fmt.Printf("N=%d: [%s]\n", n, strings.Join(strobogrammatic(n), ", "))
	}
}
