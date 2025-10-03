// Day 362: Strobogrammatic numbers of N digits.
// Recursively build from outside in using rotatable digit pairs; skip leading 0.
// Time O(output size), Space O(N) recursion depth.
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
			if n == total && p[0] == '0' {
				continue
			}
			res = append(res, string(p[0])+s+string(p[1]))
		}
	}
	return res
}

func strobogrammatic(n int) []string { return build(n, n) }

func main() {
	n := 2
	fmt.Printf("N=%d: %s\n", n, strings.Join(strobogrammatic(n), " "))
}
