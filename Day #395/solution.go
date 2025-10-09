// Group anagrams: map keyed by sorted chars -> list, preserving first-seen group order.
// Time O(N*K log K), Space O(N*K).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func groupAnagrams(words []string) [][]string {
	idx := make(map[string]int)
	groups := [][]string{}
	for _, w := range words {
		b := []byte(w)
		sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
		key := string(b)
		if g, ok := idx[key]; ok {
			groups[g] = append(groups[g], w)
		} else {
			idx[key] = len(groups)
			groups = append(groups, []string{w})
		}
	}
	return groups
}

func main() {
	input := []string{"eat", "ate", "apt", "pat", "tea", "now"}
	groups := groupAnagrams(input)
	parts := make([]string, len(groups))
	for i, g := range groups {
		quoted := make([]string, len(g))
		for j, w := range g {
			quoted[j] = "'" + w + "'"
		}
		parts[i] = "[" + strings.Join(quoted, ", ") + "]"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
