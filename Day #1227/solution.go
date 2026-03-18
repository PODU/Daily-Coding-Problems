// Graph k-colorability via backtracking: assign colors 1..k to vertices in order,
// skipping conflicts. Time O(k^n) worst case, Space O(n).
package main

import "fmt"

func safe(v int, g [][]int, color []int, c int) bool {
	for i := range g {
		if g[v][i] == 1 && color[i] == c {
			return false
		}
	}
	return true
}

func colorize(v int, g [][]int, k int, color []int) bool {
	if v == len(g) {
		return true
	}
	for c := 1; c <= k; c++ {
		if safe(v, g, color, c) {
			color[v] = c
			if colorize(v+1, g, k, color) {
				return true
			}
			color[v] = 0
		}
	}
	return false
}

func isKColorable(g [][]int, k int) bool {
	return colorize(0, g, k, make([]int, len(g)))
}

func main() {
	g := [][]int{{0, 1, 1}, {1, 0, 1}, {1, 1, 0}}
	fmt.Println(isKColorable(g, 2))
	fmt.Println(isKColorable(g, 3))
}
