// Generate all valid IPv4 addresses by backtracking: place 3 dots, each
// segment len 1..3, value 0..255, no leading zeros. Time O(1) (bounded).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func restoreIp(s string) []string {
	var res []string
	var cur []string

	var backtrack func(start, part int)
	backtrack = func(start, part int) {
		if part == 4 {
			if start == len(s) {
				res = append(res, strings.Join(cur, "."))
			}
			return
		}
		for length := 1; length <= 3 && start+length <= len(s); length++ {
			seg := s[start : start+length]
			if len(seg) > 1 && seg[0] == '0' {
				break
			}
			v, _ := strconv.Atoi(seg)
			if v > 255 {
				break
			}
			cur = append(cur, seg)
			backtrack(start+length, part+1)
			cur = cur[:len(cur)-1]
		}
	}

	backtrack(0, 0)
	return res
}

func main() {
	s := "2542540123"
	res := restoreIp(s)

	parts := make([]string, len(res))
	for i, ip := range res {
		parts[i] = "'" + ip + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
