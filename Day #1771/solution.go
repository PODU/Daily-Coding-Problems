// Day 1771: Stack-based "basic calculator" for +,-,parentheses,single digits,unary sign.
// Single left-to-right pass with a sign/result stack. Time: O(n), Space: O(n).
package main

import "fmt"

func evaluate(s string) int {
	result := 0
	sign := 1
	type frame struct{ res, sgn int }
	var stack []frame
	for _, c := range s {
		switch {
		case c >= '0' && c <= '9':
			result += sign * int(c-'0')
			sign = 1
		case c == '+':
			sign = 1
		case c == '-':
			sign = -1
		case c == '(':
			stack = append(stack, frame{result, sign})
			result = 0
			sign = 1
		case c == ')':
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			result = top.res + top.sgn*result
			sign = 1
		}
	}
	return result
}

func main() {
	fmt.Println(evaluate("-1 + (2 + 3)"))
}
