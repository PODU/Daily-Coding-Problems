// Subset Sum: boolean DP over reachable sums; reconstruct one subset by backtracking.
// Time O(n*k), Space O(n*k).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func subsetSum(S []int, k int) []int {
	n := len(S)
	reach := make([][]bool, n+1)
	for i := range reach {
		reach[i] = make([]bool, k+1)
	}
	reach[0][0] = true
	for i := 1; i <= n; i++ {
		for s := 0; s <= k; s++ {
			if reach[i-1][s] {
				reach[i][s] = true
			}
			if s >= S[i-1] && reach[i-1][s-S[i-1]] {
				reach[i][s] = true
			}
		}
	}
	if !reach[n][k] {
		return nil
	}
	chosen := []int{}
	s := k
	for i := n; i >= 1; i-- {
		if s >= S[i-1] && reach[i-1][s-S[i-1]] {
			chosen = append(chosen, S[i-1])
			s -= S[i-1]
		}
	}
	return chosen
}

func main() {
	S := []int{12, 1, 61, 5, 9, 2}
	k := 24
	sub := subsetSum(S, k)
	parts := make([]string, len(sub))
	total := 0
	for i, x := range sub {
		parts[i] = strconv.Itoa(x)
		total += x
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
	fmt.Println("Sum =", total)
}
