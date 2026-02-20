// Day 1104: Phone digit -> letters combinations via backtracking.
// Time: O(prod of choices * len). Space: O(len) recursion.
package main

import "fmt"

func letterCombos(mapping map[byte][]byte, digits string) []string {
	if len(digits) == 0 {
		return []string{}
	}
	var out []string
	var dfs func(i int, cur []byte)
	dfs = func(i int, cur []byte) {
		if i == len(digits) {
			out = append(out, string(cur))
			return
		}
		for _, c := range mapping[digits[i]] {
			dfs(i+1, append(cur, c))
		}
	}
	dfs(0, []byte{})
	return out
}

func main() {
	mapping := map[byte][]byte{'2': {'a', 'b', 'c'}, '3': {'d', 'e', 'f'}}
	fmt.Println(letterCombos(mapping, "23"))
	// [ad ae af bd be bf cd ce cf]
}
