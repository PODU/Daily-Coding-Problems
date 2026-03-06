// Goldbach: iterate a from 2 upward, return first (a, n-a) both prime (lexicographically smallest).
// Time: O(n*sqrt(n)) worst, Space: O(1).
package main

import "fmt"

func isPrime(x int) bool {
	if x < 2 {
		return false
	}
	for i := 2; i*i <= x; i++ {
		if x%i == 0 {
			return false
		}
	}
	return true
}

func goldbach(n int) (int, int) {
	for a := 2; a <= n/2; a++ {
		if isPrime(a) && isPrime(n-a) {
			return a, n - a
		}
	}
	return -1, -1
}

func main() {
	n := 4
	a, b := goldbach(n)
	fmt.Printf("%d + %d = %d\n", a, b, n)
}
