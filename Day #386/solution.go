// Sort characters by descending frequency (ties: first-occurrence order).
// Time: O(n log d), Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func frequencySort(s string) string {
	cnt := map[rune]int{}
	first := map[rune]int{}
	for i, c := range s {
		cnt[c]++
		if _, ok := first[c]; !ok {
			first[c] = i
		}
	}
	chars := make([]rune, 0, len(cnt))
	for c := range cnt {
		chars = append(chars, c)
	}
	sort.Slice(chars, func(i, j int) bool {
		a, b := chars[i], chars[j]
		if cnt[a] != cnt[b] {
			return cnt[a] > cnt[b]
		}
		return first[a] < first[b]
	})
	var out strings.Builder
	for _, c := range chars {
		out.WriteString(strings.Repeat(string(c), cnt[c]))
	}
	return out.String()
}

func main() {
	fmt.Println(frequencySort("tweet"))
}
