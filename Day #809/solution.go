// Day 809: Check balanced round/curly/square brackets using a stack.
// Push opens, match closes against stack top. Time O(N), Space O(N).
package main

import "fmt"

func isBalanced(s string) bool {
	var stack []rune
	match := map[rune]rune{')': '(', ']': '[', '}': '{'}
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
	fmt.Println(isBalanced("([])[]({})")) // true
	fmt.Println(isBalanced("([)]"))        // false
	fmt.Println(isBalanced("((()"))        // false
}
