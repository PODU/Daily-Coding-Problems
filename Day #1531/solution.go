// Rearrange string so no two adjacent chars equal.
// Greedy: place chars by descending frequency into even indices then odd indices.
// Time O(n + k log k), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func reorganize(s string) (string, bool) {
	n := len(s)
	cnt := make(map[byte]int)
	for i := 0; i < n; i++ {
		cnt[s[i]]++
	}
	maxc := 0
	for _, v := range cnt {
		if v > maxc {
			maxc = v
		}
	}
	if maxc > (n+1)/2 {
		return "", false
	}
	type pair struct {
		ch byte
		c  int
	}
	var ps []pair
	for k, v := range cnt {
		ps = append(ps, pair{k, v})
	}
	sort.Slice(ps, func(i, j int) bool {
		if ps[i].c != ps[j].c {
			return ps[i].c > ps[j].c
		}
		return ps[i].ch < ps[j].ch
	})
	res := make([]byte, n)
	idx := 0
	for _, p := range ps {
		for j := 0; j < p.c; j++ {
			res[idx] = p.ch
			idx += 2
			if idx >= n {
				idx = 1
			}
		}
	}
	return string(res), true
}

func main() {
	r, ok := reorganize("aaabbc")
	if !ok {
		fmt.Println("None")
	} else {
		fmt.Println(r)
	}
}
