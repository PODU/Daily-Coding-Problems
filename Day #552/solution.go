// Domino + Tromino tiling of 2xN: f(n)=2*f(n-1)+f(n-3), f(0)=1,f(1)=1,f(2)=2.
// O(n) time, O(1) space.
package main

import "fmt"

func tilings(n int) int64 {
	if n == 0 {
		return 1
	}
	if n == 1 {
		return 1
	}
	if n == 2 {
		return 2
	}
	var a, b, c int64 = 1, 1, 2
	cur := c
	for i := 3; i <= n; i++ {
		cur = 2*c + a
		a, b, c = b, c, cur
	}
	_ = b
	return cur
}

func main() {
	N := 4
	fmt.Println(tilings(N)) // 11
}
