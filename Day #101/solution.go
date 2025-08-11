// Day 101: Goldbach. Sieve primes up to n, then the smallest prime a with n-a
// prime gives the lexicographically smallest pair. O(n log log n).
package main

import "fmt"

func goldbach(n int) (int, int) {
	composite := make([]bool, n+1)
	for i := 2; i*i <= n; i++ {
		if !composite[i] {
			for j := i * i; j <= n; j += i {
				composite[j] = true
			}
		}
	}
	for a := 2; a <= n/2; a++ {
		if !composite[a] && !composite[n-a] {
			return a, n - a
		}
	}
	return -1, -1
}

func main() {
	a, b := goldbach(4)
	fmt.Printf("%d + %d = %d\n", a, b, a+b) // 2 + 2 = 4
}
