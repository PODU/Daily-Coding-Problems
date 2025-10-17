// Day 449: Goldbach pair. Scan a from 2 upward; the first a where a and n-a are
// both prime gives the lexicographically smallest pair. O(n*sqrt(n)) worst case.
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
	fmt.Printf("%d + %d = %d\n", a, b, n) // 2 + 2 = 4
}
