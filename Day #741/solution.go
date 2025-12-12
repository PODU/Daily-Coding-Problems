// Evaluate +/- expression with parentheses using a sign stack (Basic Calculator).
// Single linear scan; parentheses handled by pushing the running result and sign.
// Time: O(n), Space: O(n).
package main

import "fmt"

func evaluate(s string) int {
	result, sign := 0, 1
	var stack []int
	i, n := 0, len(s)
	isDigit := func(b byte) bool { return b >= '0' && b <= '9' }
	for i < n {
		c := s[i]
		if isDigit(c) {
			num := 0
			for i < n && isDigit(s[i]) {
				num = num*10 + int(s[i]-'0')
				i++
			}
			result += sign * num
			continue
		} else if c == '+' {
			sign = 1
		} else if c == '-' {
			sign = -1
		} else if c == '(' {
			stack = append(stack, result, sign)
			result, sign = 0, 1
		} else if c == ')' {
			s2 := stack[len(stack)-1]
			r2 := stack[len(stack)-2]
			stack = stack[:len(stack)-2]
			result = r2 + s2*result
		}
		i++
	}
	return result
}

func main() {
	fmt.Println(evaluate("-1 + (2 + 3)")) // 4
}
