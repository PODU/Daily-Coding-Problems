// Balanced brackets via stack. Time O(n), Space O(n).
package main

import "fmt"

func isBalanced(s string) bool {
	match := map[rune]rune{')': '(', ']': '[', '}': '{'}
	stack := []rune{}
	for _, c := range s {
		switch c {
		case '(', '[', '{':
			stack = append(stack, c)
		case ')', ']', '}':
			if len(stack) == 0 || stack[len(stack)-1] != match[c] {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}
	return len(stack) == 0
}

func main() {
	_ = isBalanced("([)]")
	_ = isBalanced("((()")
	if isBalanced("([])[]({})") {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
