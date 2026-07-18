// Min swaps to seat couples together. Union couples in each seat-pair; answer is
// N - (#connected components). O(N * alpha(N)).
package main

import "fmt"

var par []int

func find(x int) int {
	for par[x] != x {
		par[x] = par[par[x]]
		x = par[x]
	}
	return x
}

func minSwaps(row []int) int {
	n := len(row) / 2
	par = make([]int, n)
	for i := range par {
		par[i] = i
	}
	comps := n
	for i := 0; i < n; i++ {
		ra, rb := find(row[2*i]/2), find(row[2*i+1]/2)
		if ra != rb {
			par[ra] = rb
			comps--
		}
	}
	return n - comps
}

func main() {
	row := []int{0, 2, 1, 3} // couples are (0,1) and (2,3)
	fmt.Println(minSwaps(row)) // 1
}
