// Spiral matrix traversal using four boundary pointers (top,bottom,left,right). O(N*M) time, O(1) extra space.
package main

import "fmt"

func main() {
	mat := [][]int{
		{1, 2, 3, 4, 5},
		{6, 7, 8, 9, 10},
		{11, 12, 13, 14, 15},
		{16, 17, 18, 19, 20},
	}
	top, bottom := 0, len(mat)-1
	left, right := 0, len(mat[0])-1
	for top <= bottom && left <= right {
		for j := left; j <= right; j++ {
			fmt.Println(mat[top][j])
		}
		top++
		for i := top; i <= bottom; i++ {
			fmt.Println(mat[i][right])
		}
		right--
		if top <= bottom {
			for j := right; j >= left; j-- {
				fmt.Println(mat[bottom][j])
			}
			bottom--
		}
		if left <= right {
			for i := bottom; i >= top; i-- {
				fmt.Println(mat[i][left])
			}
			left++
		}
	}
}
