// Reorganize string: count freqs, if max > (n+1)/2 impossible; sort chars by freq desc (tie by char),
// fill even indices first then odd. Time O(n log k), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func reorganize(s string) (string, bool) {
	n := len(s)
	cnt := map[byte]int{}
	for i := 0; i < n; i++ {
		cnt[s[i]]++
	}
	for _, v := range cnt {
		if v > (n+1)/2 {
			return "", false
		}
	}
	chars := []byte{}
	for c := range cnt {
		chars = append(chars, c)
	}
	sort.Slice(chars, func(i, j int) bool {
		if cnt[chars[i]] != cnt[chars[j]] {
			return cnt[chars[i]] > cnt[chars[j]] // freq desc
		}
		return chars[i] < chars[j] // tie by char asc
	})
	res := make([]byte, n)
	idx := 0
	for _, c := range chars {
		for k := 0; k < cnt[c]; k++ {
			res[idx] = c
			idx += 2
			if idx >= n {
				idx = 1
			}
		}
	}
	return string(res), true
}

func main() {
	if r, ok := reorganize("aaabbc"); ok {
		fmt.Println(r)
	} else {
		fmt.Println("None")
	}
	if r, ok := reorganize("aaab"); ok {
		fmt.Println(r)
	} else {
		fmt.Println("None")
	}
}
