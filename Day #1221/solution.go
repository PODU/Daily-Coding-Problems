// Iterative rolling Fibonacci: two variables, O(n) time, O(1) space.
// fib(0)=0, fib(1)=1.
package main

import "fmt"

func fib(n int) int {
	if n < 2 {
		return n
	}
	a, b := 0, 1
	for i := 2; i <= n; i++ {
		a, b = b, a+b
	}
	return b
}

func main() {
	fmt.Println(fib(10))
}
