// Strobogrammatic numbers of N digits: build recursively from outside in using rotatable
// pairs {0-0,1-1,8-8,6-9,9-6}; skip leading 0 for N>1. Time O(5^(N/2)), Space O(N).
package main

import "fmt"

var pairs = [][2]byte{{'0', '0'}, {'1', '1'}, {'8', '8'}, {'6', '9'}, {'9', '6'}}

func build(n, total int) []string {
	if n == 0 {
		return []string{""}
	}
	if n == 1 {
		return []string{"0", "1", "8"}
	}
	inner := build(n-2, total)
	res := []string{}
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

func strobogrammatic(n int) []string { return build(n, n) }

func main() {
	fmt.Println("N=2:", strobogrammatic(2))
	fmt.Println("N=1:", strobogrammatic(1))
}
