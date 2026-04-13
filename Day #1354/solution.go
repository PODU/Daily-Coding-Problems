// Minimum jumps to reach end. Greedy: track current reach, farthest reach, jumps. O(N) time, O(1) space.
package main

import "fmt"

func minJumps(a []int) int {
	n := len(a)
	if n <= 1 {
		return 0
	}
	jumps, curEnd, farthest := 0, 0, 0
	for i := 0; i < n-1; i++ {
		if i+a[i] > farthest {
			farthest = i + a[i]
		}
		if i == curEnd {
			jumps++
			curEnd = farthest
			if curEnd >= n-1 {
				break
			}
		}
	}
	return jumps
}

func main() {
	a := []int{6, 2, 4, 0, 5, 1, 1, 4, 2, 9}
	fmt.Println(minJumps(a))
}
