// Phone keypad letter combinations via iterative Cartesian product. O(prod*len) time, O(output) space.
package main

import (
	"fmt"
	"strings"
)

func letterCombinations(digits string, mp map[rune]string) []string {
	if digits == "" {
		return nil
	}
	res := []string{""}
	for _, d := range digits {
		var next []string
		for _, pre := range res {
			for _, c := range mp[d] {
				next = append(next, pre+string(c))
			}
		}
		res = next
	}
	return res
}

func main() {
	mp := map[rune]string{'2': "abc", '3': "def"}
	res := letterCombinations("23", mp)
	parts := make([]string, len(res))
	for i, w := range res {
		parts[i] = "\"" + w + "\""
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
