// Day 974: Evaluate expression with parentheses, digits, +/- (no eval).
// Approach: single scan with a sign/result stack. Time O(n), Space O(n).
package main

import "fmt"

func evaluate(s string) int64 {
	var result, num int64
	sign := int64(1)
	var st []int64 // alternating result, sign
	for _, c := range s {
		switch {
		case c >= '0' && c <= '9':
			num = num*10 + int64(c-'0')
		case c == '+':
			result += sign * num
			num = 0
			sign = 1
		case c == '-':
			result += sign * num
			num = 0
			sign = -1
		case c == '(':
			st = append(st, result, sign)
			result = 0
			sign = 1
		case c == ')':
			result += sign * num
			num = 0
			prevSign := st[len(st)-1]
			prevResult := st[len(st)-2]
			st = st[:len(st)-2]
			result = prevResult + prevSign*result
			sign = 1
		}
	}
	result += sign * num
	return result
}

func main() {
	fmt.Println(evaluate("-1 + (2 + 3)")) // 4
}
