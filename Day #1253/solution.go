// Balanced brackets via stack: push openers, match closers to top. O(n) time, O(n) space.
package main

import "fmt"

func isBalanced(s string) bool {
	st := []rune{}
	pair := map[rune]rune{')': '(', ']': '[', '}': '{'}
	for _, c := range s {
		if c == '(' || c == '[' || c == '{' {
			st = append(st, c)
		} else if open, ok := pair[c]; ok {
			if len(st) == 0 || st[len(st)-1] != open {
				return false
			}
			st = st[:len(st)-1]
		}
	}
	return len(st) == 0
}

func main() {
	fmt.Println(isBalanced("([])[]({})"))
	fmt.Println(isBalanced("([)]"))
}
