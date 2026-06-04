// Goldbach pair: sieve up to n, scan a from 2; first a where a and n-a prime.
// Smallest a => lexicographically smallest [a,b]. Time O(n log log n), Space O(n).
package main

import "fmt"

func goldbach(n int) (int, int) {
	isPrime := make([]bool, n+1)
	for i := range isPrime {
		isPrime[i] = true
	}
	isPrime[0], isPrime[1] = false, false
	for i := 2; i*i <= n; i++ {
		if isPrime[i] {
			for j := i * i; j <= n; j += i {
				isPrime[j] = false
			}
		}
	}
	for a := 2; a <= n-a; a++ {
		if isPrime[a] && isPrime[n-a] {
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
