// Couples holding hands: union the two couples occupying each seat-pair. Minimum swaps =
// N - (number of connected components among couples). Time: O(N alpha), Space: O(N).
package main

import "fmt"

var parent []int
var comps int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b int) {
	a, b = find(a), find(b)
	if a != b {
		parent[a] = b
		comps--
	}
}

func minSwaps(row []int) int {
	n := len(row) / 2
	parent = make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	comps = n
	for i := 0; i < len(row); i += 2 {
		unite(row[i]/2, row[i+1]/2)
	}
	return n - comps
}

func main() {
	fmt.Println(minSwaps([]int{0, 2, 1, 3})) // 1
}
