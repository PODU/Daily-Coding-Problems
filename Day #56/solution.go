// Day 56: Graph k-colorability via backtracking.
// Time: O(k^V) worst case, Space: O(V).
package main

import "fmt"

func canColor(g [][]int, k int, color []int, v int) bool {
	n := len(g)
	if v == n {
		return true
	}
	for c := 1; c <= k; c++ {
		ok := true
		for u := 0; u < n; u++ {
			if g[v][u] == 1 && color[u] == c {
				ok = false
				break
			}
		}
		if !ok {
			continue
		}
		color[v] = c
		if canColor(g, k, color, v+1) {
			return true
		}
		color[v] = 0
	}
	return false
}

func kColorable(g [][]int, k int) bool {
	return canColor(g, k, make([]int, len(g)), 0)
}

func main() {
	// Triangle graph: needs 3 colors.
	g := [][]int{
		{0, 1, 1},
		{1, 0, 1},
		{1, 1, 0},
	}
	fmt.Println(kColorable(g, 2)) // false
	fmt.Println(kColorable(g, 3)) // true
}
