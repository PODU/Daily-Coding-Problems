// Day 1416: evaluate a +/-/parenthesized expression of single digits, no eval.
// Approach: single scan with a sign stack (Basic Calculator). O(n) time, O(n) space.
package main

import "fmt"

func evaluate(s string) int {
	result, sign := 0, 1
	stack := []int{1}
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
			result += sign * stack[len(stack)-1] * num
			continue
		} else if c == '+' {
			sign = 1
		} else if c == '-' {
			sign = -1
		} else if c == '(' {
			stack = append(stack, sign*stack[len(stack)-1])
			sign = 1
		} else if c == ')' {
			stack = stack[:len(stack)-1]
		}
		i++
	}
	return result
}

func main() {
	fmt.Println(evaluate("-1 + (2 + 3)")) // 4
}
