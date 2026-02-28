// Phone keypad letter combinations via backtracking. O(prod of choices) time.
package main

import (
	"fmt"
	"strings"
)

var M = map[byte]string{
	'2': "abc", '3': "def", '4': "ghi", '5': "jkl",
	'6': "mno", '7': "pqrs", '8': "tuv", '9': "wxyz",
}

func letterCombinations(digits string) []string {
	if len(digits) == 0 {
		return []string{}
	}
	out := []string{}
	cur := []byte{}
	var backtrack func(i int)
	backtrack = func(i int) {
		if i == len(digits) {
			out = append(out, string(cur))
			return
		}
		for _, c := range []byte(M[digits[i]]) {
			cur = append(cur, c)
			backtrack(i + 1)
			cur = cur[:len(cur)-1]
		}
	}
	backtrack(0)
	return out
}

func main() {
	res := letterCombinations("23")
	fmt.Println("[" + strings.Join(res, ", ") + "]")
}
