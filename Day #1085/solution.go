// Generate valid IP addresses via backtracking over 4 octets (each 0-255, no leading zeros).
// Time O(1) (bounded by string length <= 12), Space O(1) extra.
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
	n, _ := strconv.Atoi(seg)
	return n <= 255
}

func restoreIPAddresses(s string) []string {
	var res []string
	var backtrack func(start, part int, cur []string)
	backtrack = func(start, part int, cur []string) {
		if part == 4 {
			if start == len(s) {
				res = append(res, strings.Join(cur, "."))
			}
			return
		}
		for l := 1; l <= 3 && start+l <= len(s); l++ {
			seg := s[start : start+l]
			if valid(seg) {
				backtrack(start+l, part+1, append(cur, seg))
			}
		}
	}
	backtrack(0, 0, []string{})
	return res
}

func main() {
	res := restoreIPAddresses("2542540123")
	parts := make([]string, len(res))
	for i, r := range res {
		parts[i] = "'" + r + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
