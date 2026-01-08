// Evaluate Reverse Polish Notation with a stack. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strconv"
)

func evalRPN(tokens []string) int64 {
	st := []int64{}
	for _, t := range tokens {
		switch t {
		case "+", "-", "*", "/":
			n := len(st)
			a, b := st[n-2], st[n-1]
			st = st[:n-2]
			var r int64
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
			v, _ := strconv.ParseInt(t, 10, 64)
			st = append(st, v)
		}
	}
	return st[len(st)-1]
}

func main() {
	tokens := []string{"15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"}
	fmt.Println(evalRPN(tokens))
}
