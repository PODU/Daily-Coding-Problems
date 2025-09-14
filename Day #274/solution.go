// Day 274: Evaluate string of (), single digits, +/- without eval.
// Stack-based sign tracking. Time O(N), Space O(N).
package main

import "fmt"

func evaluate(s string) int {
	result, sign := 0, 1
	var st []int
	for _, c := range s {
		switch {
		case c >= '0' && c <= '9':
			result += sign * int(c-'0')
		case c == '+':
			sign = 1
		case c == '-':
			sign = -1
		case c == '(':
			st = append(st, result, sign)
			result, sign = 0, 1
		case c == ')':
			s2 := st[len(st)-1]
			prev := st[len(st)-2]
			st = st[:len(st)-2]
			result = prev + s2*result
		}
	}
	return result
}

func main() {
	fmt.Println(evaluate("-1 + (2 + 3)")) // 4
}
