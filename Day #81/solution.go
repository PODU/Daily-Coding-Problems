// Day 81: Phone-number letter combinations via iterative cartesian product.
// Time O(prod of letters * len), Space O(output).
package main

import "fmt"

func letterCombinations(mapping map[byte][]string, digits string) []string {
	if len(digits) == 0 {
		return []string{}
	}
	res := []string{""}
	for i := 0; i < len(digits); i++ {
		d := digits[i]
		next := []string{}
		for _, prefix := range res {
			for _, letter := range mapping[d] {
				next = append(next, prefix+letter)
			}
		}
		res = next
	}
	return res
}

func main() {
	mapping := map[byte][]string{
		'2': {"a", "b", "c"}, '3': {"d", "e", "f"}, '4': {"g", "h", "i"},
		'5': {"j", "k", "l"}, '6': {"m", "n", "o"}, '7': {"p", "q", "r", "s"},
		'8': {"t", "u", "v"}, '9': {"w", "x", "y", "z"},
	}
	fmt.Println(letterCombinations(mapping, "23"))
	// [ad ae af bd be bf cd ce cf]
}
