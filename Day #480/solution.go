// Word break reconstruction via memoized DP: for each suffix, try each prefix
// word and recurse. Time: O(n^2 * L) with memo, Space: O(n^2).
package main

import (
	"fmt"
	"strings"
)

func reconstruct(words []string, s string) ([]string, bool) {
	dict := make(map[string]bool)
	for _, w := range words {
		dict[w] = true
	}
	memo := make(map[int][]string)
	ok := make(map[int]bool)

	var solve func(start int) ([]string, bool)
	solve = func(start int) ([]string, bool) {
		if start == len(s) {
			return []string{}, true
		}
		if v, seen := memo[start]; seen {
			return v, ok[start]
		}
		for end := start + 1; end <= len(s); end++ {
			word := s[start:end]
			if dict[word] {
				if rest, found := solve(end); found {
					res := append([]string{word}, rest...)
					memo[start] = res
					ok[start] = true
					return res, true
				}
			}
		}
		memo[start] = nil
		ok[start] = false
		return nil, false
	}

	return solve(0)
}

func fmtRes(res []string, found bool) string {
	if !found {
		return "null"
	}
	parts := make([]string, len(res))
	for i, w := range res {
		parts[i] = "'" + w + "'"
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	r1, ok1 := reconstruct([]string{"quick", "brown", "the", "fox"}, "thequickbrownfox")
	fmt.Println(fmtRes(r1, ok1))
	r2, ok2 := reconstruct([]string{"bed", "bath", "bedbath", "and", "beyond"}, "bedbathandbeyond")
	fmt.Println(fmtRes(r2, ok2))
}
