// Approach: deterministic single linear scan validating sign/digits/dot/exponent.
// Time O(n), Space O(1).
package main

import "fmt"

func isDigit(c byte) bool { return c >= '0' && c <= '9' }

func isNumber(s string) bool {
	n := len(s)
	if n == 0 {
		return false
	}
	i := 0
	if s[i] == '+' || s[i] == '-' {
		i++
	}
	digits := false
	dot := false
	for i < n && (isDigit(s[i]) || s[i] == '.') {
		if s[i] == '.' {
			if dot {
				return false
			}
			dot = true
		} else {
			digits = true
		}
		i++
	}
	if !digits {
		return false
	}
	if i < n && (s[i] == 'e' || s[i] == 'E') {
		i++
		if i < n && (s[i] == '+' || s[i] == '-') {
			i++
		}
		expDigits := false
		for i < n && isDigit(s[i]) {
			expDigits = true
			i++
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
		if isNumber(t) {
			fmt.Println("true")
		} else {
			fmt.Println("false")
		}
	}
}
