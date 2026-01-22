// Day 939: Print an N x M matrix in clockwise spiral order.
// Shrink four boundaries layer by layer. Time O(N*M), Space O(1) extra.
package main

import "fmt"

func spiral(m [][]int) {
	if len(m) == 0 {
		return
	}
	top, bottom := 0, len(m)-1
	left, right := 0, len(m[0])-1
	for top <= bottom && left <= right {
		for c := left; c <= right; c++ {
			fmt.Println(m[top][c])
		}
		top++
		for r := top; r <= bottom; r++ {
			fmt.Println(m[r][right])
		}
		right--
		if top <= bottom {
			for c := right; c >= left; c-- {
				fmt.Println(m[bottom][c])
			}
			bottom--
		}
		if left <= right {
			for r := bottom; r >= top; r-- {
				fmt.Println(m[r][left])
			}
			left++
		}
	}
}

func main() {
	matrix := [][]int{
		{1, 2, 3, 4, 5},
		{6, 7, 8, 9, 10},
		{11, 12, 13, 14, 15},
		{16, 17, 18, 19, 20},
	}
	spiral(matrix) // 1 2 3 4 5 10 15 20 19 18 17 16 11 6 7 8 9 14 13 12
}
