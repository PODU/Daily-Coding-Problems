// Spiral traversal by peeling outer ring (top,right,bottom,left) toward center.
// Time O(N*M), Space O(1) extra.
package main

import "fmt"

func spiral(m [][]int) {
	if len(m) == 0 {
		return
	}
	top, bottom, left, right := 0, len(m)-1, 0, len(m[0])-1
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
	m := [][]int{
		{1, 2, 3, 4, 5},
		{6, 7, 8, 9, 10},
		{11, 12, 13, 14, 15},
		{16, 17, 18, 19, 20},
	}
	spiral(m)
}
