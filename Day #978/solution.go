// Valid number check via single-pass finite-state parser.
// Grammar: [sign] digits [. digits] | [sign] [digits] . digits, then optional (e/E [sign] digits).
// Time: O(n); Space: O(1).
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
	if !digitsBefore && !digitsAfter { // mantissa needs a digit
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
		if !expDigits { // exponent needs a digit
			return false
		}
	}
	return i == n // no trailing junk
}

func main() {
	valid := []string{"10", "-10", "10.1", "-10.1", "1e5"}
	invalid := []string{"a", "x 1", "a -2", "-", "", " "}
	for _, s := range append(valid, invalid...) {
		fmt.Printf("\"%s\" -> %t\n", s, isNumber(s))
	}
}
