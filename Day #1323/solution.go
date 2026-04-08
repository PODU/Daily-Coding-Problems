// Day 1323: Min lowering cost to form a pyramid (rise by 1 to a peak, fall by 1, unit ends).
// left[i]=min(a[i],left[i-1]+1), right[i] symmetric; best peak h=max(min(left,right)); pyramid sums to h^2.
// Cost = sum(a) - h^2. O(n) time, O(n) space.
package main

import "fmt"

func min(a, b int) int { if a < b { return a }; return b }
func abs(x int) int { if x < 0 { return -x }; return x }

func main() {
	a := []int{1, 1, 3, 3, 2, 1}
	n := len(a)
	left := make([]int, n)
	right := make([]int, n)
	for i := 0; i < n; i++ {
		prev := 0
		if i > 0 {
			prev = left[i-1]
		}
		left[i] = min(a[i], prev+1)
	}
	for i := n - 1; i >= 0; i-- {
		nxt := 0
		if i < n-1 {
			nxt = right[i+1]
		}
		right[i] = min(a[i], nxt+1)
	}

	h, peak := 0, 0
	for i := 0; i < n; i++ {
		hi := min(left[i], right[i])
		if hi > h {
			h, peak = hi, i
		}
	}

	target := make([]int, n)
	total := 0
	for i := 0; i < n; i++ {
		total += a[i]
		d := abs(i - peak)
		if d < h {
			target[i] = h - d
		}
	}
	cost := total - h*h
	fmt.Println(cost)   // 2
	fmt.Println(target) // [0 1 2 3 2 1]
}
