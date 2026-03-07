// Day 1174: Decide whether a string is a valid number.
// Single linear scan: optional sign, integer/fraction digits, optional exponent.
// Time O(N), Space O(1).
package main

import "fmt"

func isNumber(s string) bool {
	n, i := len(s), 0
	if n == 0 {
		return false
	}
	digit := func(k int) bool { return k < n && s[k] >= '0' && s[k] <= '9' }
	if i < n && (s[i] == '+' || s[i] == '-') {
		i++
	}
	before, after := false, false
	for digit(i) {
		i++
		before = true
	}
	if i < n && s[i] == '.' {
		i++
		for digit(i) {
			i++
			after = true
		}
	}
	if !before && !after {
		return false
	}
	if i < n && (s[i] == 'e' || s[i] == 'E') {
		i++
		if i < n && (s[i] == '+' || s[i] == '-') {
			i++
		}
		exp := false
		for digit(i) {
			i++
			exp = true
		}
		if !exp {
			return false
		}
	}
	return i == n
}

func main() {
	tests := []string{"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"}
	for _, t := range tests {
		res := "false"
		if isNumber(t) {
			res = "true"
		}
		fmt.Printf("\"%s\" -> %s\n", t, res)
	}
}
