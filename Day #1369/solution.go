// Restore IP addresses via backtracking over 4 octets. Time O(1) (<=3^3 splits),
// Space O(1) recursion depth. Each octet in [0,255], no leading zeros (except "0").
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

func restore(s string) []string {
	var res []string
	var bt func(start, part int, cur []string)
	bt = func(start, part int, cur []string) {
		if part == 4 {
			if start == len(s) {
				res = append(res, strings.Join(cur, "."))
			}
			return
		}
		for l := 1; l <= 3 && start+l <= len(s); l++ {
			seg := s[start : start+l]
			if valid(seg) {
				bt(start+l, part+1, append(cur, seg))
			}
		}
	}
	bt(0, 0, []string{})
	return res
}

func main() {
	res := restore("2542540123")
	parts := make([]string, len(res))
	for i, r := range res {
		parts[i] = "'" + r + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
