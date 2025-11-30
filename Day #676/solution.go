// Valid number validator via single-pass state machine (sign/int/dot/frac/exp).
// Time: O(n) over string length, Space: O(1).
package main

import "fmt"

func isDigit(c byte) bool { return c >= '0' && c <= '9' }

func isValidNumber(s string) bool {
	i, n := 0, len(s)
	if n == 0 {
		return false
	}
	if s[i] == '+' || s[i] == '-' {
		i++
	}
	digitsBefore, digitsAfter := false, false
	for i < n && isDigit(s[i]) {
		i++
		digitsBefore = true
	}
	if i < n && s[i] == '.' {
		i++
		for i < n && isDigit(s[i]) {
			i++
			digitsAfter = true
		}
	}
	if !digitsBefore && !digitsAfter {
		return false
	}
	if i < n && (s[i] == 'e' || s[i] == 'E') {
		i++
		if i < n && (s[i] == '+' || s[i] == '-') {
			i++
		}
		expDigits := false
		for i < n && isDigit(s[i]) {
			i++
			expDigits = true
		}
		if !expDigits {
			return false
		}
	}
	return i == n
}

func main() {
	tests := []string{"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"}
	for _, t := range tests {
		res := "False"
		if isValidNumber(t) {
			res = "True"
		}
		fmt.Printf("\"%s\" -> %s\n", t, res)
	}
}
