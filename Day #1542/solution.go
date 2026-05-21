// Balanced brackets check using a stack.
// Time O(n), Space O(n).
package main

import "fmt"

func isBalanced(s string) bool {
	match := map[rune]rune{')': '(', ']': '[', '}': '{'}
	var st []rune
	for _, c := range s {
		switch c {
		case '(', '[', '{':
			st = append(st, c)
		case ')', ']', '}':
			if len(st) == 0 || st[len(st)-1] != match[c] {
				return false
			}
			st = st[:len(st)-1]
		}
	}
	return len(st) == 0
}

func main() {
	fmt.Println(isBalanced("([])[]({})"))
}
