// Day 1857: Evaluate Reverse Polish Notation.
// Stack: push numbers, pop two on operator and apply. O(n) time, O(n) space.
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
			b := st[len(st)-1]
			a := st[len(st)-2]
			st = st[:len(st)-2]
			var r int64
			switch t {
			case "+":
				r = a + b
			case "-":
				r = a - b
			case "*":
				r = a * b
			case "/":
				r = a / b // Go integer division truncates toward zero
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
	fmt.Println(evalRPN(tokens)) // 5
}
