// Day 96: All permutations via backtracking on the remaining elements, yielding
// lexicographic order. O(n*n!) time.
package main

import "fmt"

func backtrack(path, rem []int, res *[][]int) {
	if len(rem) == 0 {
		cp := make([]int, len(path))
		copy(cp, path)
		*res = append(*res, cp)
		return
	}
	for i := range rem {
		next := make([]int, 0, len(rem)-1)
		next = append(next, rem[:i]...)
		next = append(next, rem[i+1:]...)
		backtrack(append(path, rem[i]), next, res)
	}
}

func main() {
	var res [][]int
	backtrack([]int{}, []int{1, 2, 3}, &res)
	fmt.Println(res)
	// [[1 2 3] [1 3 2] [2 1 3] [2 3 1] [3 1 2] [3 2 1]]
}
