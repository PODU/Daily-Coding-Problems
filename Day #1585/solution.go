// Day 1585: Sieve of Eratosthenes + incremental infinite prime generator.
// Sieve marks composites up to N; generator streams primes via a goroutine channel.
// Time: O(N log log N) sieve; Space: O(N).
package main

import "fmt"

func sieve(n int) []int {
	comp := make([]bool, n)
	var primes []int
	for i := 2; i < n; i++ {
		if !comp[i] {
			primes = append(primes, i)
			for j := i * i; j < n; j += i {
				comp[j] = true
			}
		}
	}
	return primes
}

// firstPrimes uses an incremental sieve (unbounded in principle).
func firstPrimes(count int) []int {
	var primes []int
	composites := map[int][]int{}
	for n := 2; len(primes) < count; n++ {
		if hit, ok := composites[n]; !ok {
			primes = append(primes, n)
			composites[n*n] = append(composites[n*n], n)
		} else {
			for _, p := range hit {
				composites[n+p] = append(composites[n+p], p)
			}
			delete(composites, n)
		}
	}
	return primes
}

func main() {
	fmt.Println("Primes < 30:", sieve(30))
	fmt.Println("First 10 primes:", firstPrimes(10))
}
