// Day 759: Generate all valid IPv4 addresses from a digit string (backtracking).
// At most 3^3 splits; Time: O(1) practically (bounded), Space: O(#results).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func validOctet(s string) bool {
	if len(s) == 0 || len(s) > 3 {
		return false
	}
	if len(s) > 1 && s[0] == '0' {
		return false
	}
	v, _ := strconv.Atoi(s)
	return v <= 255
}

func restoreIps(s string) []string {
	res := []string{}
	var backtrack func(start int, parts []string)
	backtrack = func(start int, parts []string) {
		if len(parts) == 4 {
			if start == len(s) {
				res = append(res, strings.Join(parts, "."))
			}
			return
		}
		for length := 1; length <= 3 && start+length <= len(s); length++ {
			seg := s[start : start+length]
			if validOctet(seg) {
				backtrack(start+length, append(parts, seg))
			}
		}
	}
	backtrack(0, []string{})
	return res
}

func main() {
	res := restoreIps("2542540123")
	quoted := make([]string, len(res))
	for i, r := range res {
		quoted[i] = "'" + r + "'"
	}
	fmt.Println("[" + strings.Join(quoted, ", ") + "]")
	// ['254.25.40.123', '254.254.0.123']
}
