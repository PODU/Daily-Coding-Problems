// Iterative Fibonacci with two rolling variables. O(n) time, O(1) space.
package main

import "fmt"

func fib(n int) int {
	if n == 0 { return 0 }
	a, b := 0, 1
	for i := 2; i <= n; i++ { a, b = b, a+b }
	return b
}

func main() {
	for i := 0; i <= 10; i++ {
		if i > 0 { fmt.Print(" ") }
		fmt.Print(fib(i))
	}
	fmt.Println()
	fmt.Printf("fib(10) = %d\n", fib(10))
}
