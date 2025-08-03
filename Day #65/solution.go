// Spiral order print via boundary shrinking (top/bottom/left/right). Time O(N*M), Space O(1) extra.
package main

import "fmt"

func main() {
	m := [][]int{
		{1, 2, 3, 4, 5},
		{6, 7, 8, 9, 10},
		{11, 12, 13, 14, 15},
		{16, 17, 18, 19, 20},
	}
	top, bottom := 0, len(m)-1
	left, right := 0, len(m[0])-1
	for top <= bottom && left <= right {
		for j := left; j <= right; j++ {
			fmt.Println(m[top][j])
		}
		top++
		for i := top; i <= bottom; i++ {
			fmt.Println(m[i][right])
		}
		right--
		if top <= bottom {
			for j := right; j >= left; j-- {
				fmt.Println(m[bottom][j])
			}
			bottom--
		}
		if left <= right {
			for i := bottom; i >= top; i-- {
				fmt.Println(m[i][left])
			}
			left++
		}
	}
}
