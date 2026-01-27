// Day 972: Rearrange string so no two adjacent chars match (else "None").
// Approach: place most frequent chars into even-then-odd slots. Time O(n log k), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func rearrange(s string) string {
	n := len(s)
	cnt := make(map[byte]int)
	for i := 0; i < n; i++ {
		cnt[s[i]]++
	}
	type pair struct {
		ch  byte
		c   int
	}
	var v []pair
	for ch, c := range cnt {
		v = append(v, pair{ch, c})
	}
	sort.Slice(v, func(i, j int) bool {
		if v[i].c != v[j].c {
			return v[i].c > v[j].c
		}
		return v[i].ch < v[j].ch
	})
	if v[0].c > (n+1)/2 {
		return "None"
	}
	res := make([]byte, n)
	idx := 0
	for _, p := range v {
		for k := 0; k < p.c; k++ {
			res[idx] = p.ch
			idx += 2
			if idx >= n {
				idx = 1
			}
		}
	}
	return string(res)
}

func main() {
	fmt.Println(rearrange("aaabbc")) // ababac
	fmt.Println(rearrange("aaab"))   // None
}
