// Day 451: nth Fibonacci number in O(1) space.
// Iterative rolling pair. Time O(n), Space O(1).
package main

import "fmt"

func fib(n int) int64 {
	if n < 2 {
		return int64(n)
	}
	var a, b int64 = 0, 1
	for i := 2; i <= n; i++ {
		a, b = b, a+b
	}
	return b
}

func main() {
	fmt.Println(fib(10)) // 55
}
