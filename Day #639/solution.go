// Day 639: Letter combinations of a phone number.
// Approach: iterative Cartesian product over digit->letters map.
// Time: O(4^n * n), Space: O(4^n).
package main

import "fmt"

func letterCombinations(digits string, mapping map[rune][]string) []string {
	if digits == "" {
		return []string{}
	}
	res := []string{""}
	for _, d := range digits {
		next := []string{}
		for _, prefix := range res {
			for _, ch := range mapping[d] {
				next = append(next, prefix+ch)
			}
		}
		res = next
	}
	return res
}

func main() {
	mapping := map[rune][]string{
		'2': {"a", "b", "c"}, '3': {"d", "e", "f"}, '4': {"g", "h", "i"},
		'5': {"j", "k", "l"}, '6': {"m", "n", "o"}, '7': {"p", "q", "r", "s"},
		'8': {"t", "u", "v"}, '9': {"w", "x", "y", "z"},
	}
	fmt.Println(letterCombinations("23", mapping))
}
