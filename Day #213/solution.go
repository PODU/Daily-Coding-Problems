// Day 213: Generate all valid IP addresses from a digit string.
// Approach: backtracking over the 3 dot positions; each segment 1-3 digits, 0-255, no leading zeros.
// Time O(1) effectively (length <= 12), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func valid(seg string) bool {
	if len(seg) == 0 || len(seg) > 3 {
		return false
	}
	if len(seg) > 1 && seg[0] == '0' {
		return false
	}
	v, _ := strconv.Atoi(seg)
	return v <= 255
}

func restore(s string) []string {
	res := []string{}
	var solve func(start int, parts []string)
	solve = func(start int, parts []string) {
		if len(parts) == 4 {
			if start == len(s) {
				res = append(res, strings.Join(parts, "."))
			}
			return
		}
		for l := 1; l <= 3 && start+l <= len(s); l++ {
			seg := s[start : start+l]
			if valid(seg) {
				solve(start+l, append(append([]string{}, parts...), seg))
			}
		}
	}
	solve(0, []string{})
	return res
}

func main() {
	r := restore("2542540123")
	quoted := make([]string, len(r))
	for i, x := range r {
		quoted[i] = "'" + x + "'"
	}
	fmt.Println("[" + strings.Join(quoted, ", ") + "]")
}
