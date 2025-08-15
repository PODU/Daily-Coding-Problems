// Day 123: Validate whether a string is a number (int/real/scientific).
// Single linear scan state machine. O(n) time, O(1) space.
package main

import "fmt"

func isDigit(c byte) bool { return c >= '0' && c <= '9' }

func isNumber(s string) bool {
	i, n := 0, len(s)
	if n == 0 {
		return false
	}
	if s[i] == '+' || s[i] == '-' {
		i++
	}
	digits, dots := 0, 0
	for i < n && (isDigit(s[i]) || s[i] == '.') {
		if s[i] == '.' {
			dots++
		} else {
			digits++
		}
		i++
	}
	if dots > 1 || digits == 0 {
		return false
	}
	if i < n && (s[i] == 'e' || s[i] == 'E') {
		i++
		if i < n && (s[i] == '+' || s[i] == '-') {
			i++
		}
		expd := 0
		for i < n && isDigit(s[i]) {
			expd++
			i++
		}
		if expd == 0 {
			return false
		}
	}
	return i == n
}

func main() {
	tests := []string{"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"}
	for _, t := range tests {
		fmt.Printf("\"%s\" -> %t\n", t, isNumber(t))
	}
}
