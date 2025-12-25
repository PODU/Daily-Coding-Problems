// Day 797: Goldbach - two primes summing to even n, lexicographically smallest.
// Scan a from 2 upward; first prime a with prime (n-a) gives smallest pair.
// Time O(n * sqrt(n)), Space O(1).
package main

import "fmt"

func isPrime(x int) bool {
	if x < 2 {
		return false
	}
	for d := 2; d*d <= x; d++ {
		if x%d == 0 {
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
	fmt.Printf("%d + %d = %d\n", a, b, n) // 2 + 2 = 4
}
