// Day 712: Balanced brackets check using a stack. Time O(n), space O(n).
package main

import "fmt"

func balanced(s string) bool {
	st := []rune{}
	match := map[rune]rune{')': '(', ']': '[', '}': '{'}
	for _, c := range s {
		if c == '(' || c == '[' || c == '{' {
			st = append(st, c)
		} else if open, ok := match[c]; ok {
			if len(st) == 0 || st[len(st)-1] != open {
				return false
			}
			st = st[:len(st)-1]
		}
	}
	return len(st) == 0
}

func main() {
	fmt.Println(balanced("([])[]({})"))
	fmt.Println(balanced("([)]"))
	fmt.Println(balanced("((()"))
}
