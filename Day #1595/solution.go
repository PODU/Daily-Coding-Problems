// Approach: DP min palindrome partition. pal[i][j] table O(n^2), cut[i]=min cuts for prefix,
// then reconstruct one optimal partition. Time O(n^2), Space O(n^2).
package main

import (
	"fmt"
	"math"
	"strings"
)

func minPalPartition(s string) []string {
	n := len(s)
	if n == 0 {
		return []string{}
	}
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
		pal[i][i] = true
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			if s[i] == s[j] && (length == 2 || pal[i+1][j-1]) {
				pal[i][j] = true
			}
		}
	}
	cut := make([]int, n)
	start := make([]int, n)
	for i := 0; i < n; i++ {
		if pal[0][i] {
			cut[i] = 0
			start[i] = 0
			continue
		}
		best, bj := math.MaxInt32, 0
		for j := 1; j <= i; j++ {
			if pal[j][i] && cut[j-1]+1 < best {
				best = cut[j-1] + 1
				bj = j
			}
		}
		cut[i] = best
		start[i] = bj
	}
	res := []string{}
	i := n - 1
	for i >= 0 {
		j := start[i]
		res = append(res, s[j:i+1])
		i = j - 1
	}
	for l, r := 0, len(res)-1; l < r; l, r = l+1, r-1 {
		res[l], res[r] = res[r], res[l]
	}
	return res
}

func fmtList(v []string) string {
	q := make([]string, len(v))
	for i, x := range v {
		q[i] = "\"" + x + "\""
	}
	return "[" + strings.Join(q, ", ") + "]"
}

func main() {
	fmt.Println(fmtList(minPalPartition("racecarannakayak")))
	fmt.Println(fmtList(minPalPartition("abc")))
}
