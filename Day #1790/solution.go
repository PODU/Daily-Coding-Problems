// fib(n): iterative two-variable rolling sum.
// Time O(N), Space O(1).
package main

import "fmt"

func fib(n int) int64 {
	var a, b int64 = 0, 1
	for i := 0; i < n; i++ {
		a, b = b, a+b
	}
	return a
}

func main() {
	fmt.Println(fib(10))
}
