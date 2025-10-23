// Day 481: Evaluate Reverse Polish Notation using a stack.
// Approach: push operands; on operator pop two and apply. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strconv"
)

func evalRPN(tokens []string) int {
	st := []int{}
	for _, t := range tokens {
		switch t {
		case "+", "-", "*", "/":
			n := len(st)
			a, b := st[n-2], st[n-1]
			st = st[:n-2]
			var r int
			switch t {
			case "+":
				r = a + b
			case "-":
				r = a - b
			case "*":
				r = a * b
			default:
				r = a / b
			}
			st = append(st, r)
		default:
			v, _ := strconv.Atoi(t)
			st = append(st, v)
		}
	}
	return st[len(st)-1]
}

func main() {
	tokens := []string{"15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"}
	fmt.Println(evalRPN(tokens)) // 5
}
