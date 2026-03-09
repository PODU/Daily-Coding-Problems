// Day 1183: Generate all N-digit strobogrammatic numbers (same when rotated 180).
// Recursively build from outside in using rotation pairs; drop leading zeros.
// Time O(output size), Space O(N) recursion depth.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

var pairs = [][2]byte{{'0', '0'}, {'1', '1'}, {'6', '9'}, {'8', '8'}, {'9', '6'}}
var rot = map[byte]byte{'0': '0', '1': '1', '6': '9', '8': '8', '9': '6'}

func helper(m int) []string {
	if m == 0 {
		return []string{""}
	}
	if m == 1 {
		return []string{"0", "1", "8"}
	}
	inner := helper(m - 2)
	var res []string
	for _, s := range inner {
		for _, p := range pairs {
			res = append(res, string(p[0])+s+string(p[1]))
		}
	}
	return res
}

func strobogrammatic(n int) []int {
	var out []int
	for _, s := range helper(n) {
		if (len(s) > 1 && s[0] == '0') || s == "0" {
			continue
		}
		v, _ := strconv.Atoi(s)
		out = append(out, v)
	}
	sort.Ints(out)
	return out
}

func isStrobo(s string) bool {
	for l, r := 0, len(s)-1; l <= r; l, r = l+1, r-1 {
		v, ok := rot[s[l]]
		if !ok || v != s[r] {
			return false
		}
	}
	return true
}

func main() {
	v := strobogrammatic(2)
	strs := make([]string, len(v))
	for i, x := range v {
		strs[i] = strconv.Itoa(x)
	}
	fmt.Printf("2-digit strobogrammatic numbers: [%s]\n", strings.Join(strs, ", "))
	fmt.Printf("16891 is strobogrammatic: %t\n", isStrobo("16891"))
}
