// Sieve of Eratosthenes: primes < N in O(N log log N) time, O(N) space.
// Plus an indefinite prime generator via a closure using trial division by known primes.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func sieve(n int) []int {
	comp := make([]bool, max(n, 0))
	primes := []int{}
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

// primeGen returns a closure that yields the next prime on each call.
func primeGen() func() int {
	primes := []int{}
	cand := 1
	return func() int {
		for {
			cand++
			isPrime := true
			for _, p := range primes {
				if p*p > cand {
					break
				}
				if cand%p == 0 {
					isPrime = false
					break
				}
			}
			if isPrime {
				primes = append(primes, cand)
				return cand
			}
		}
	}
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func join(nums []int) string {
	parts := make([]string, len(nums))
	for i, v := range nums {
		parts[i] = strconv.Itoa(v)
	}
	return strings.Join(parts, " ")
}

func main() {
	fmt.Println(join(sieve(100)))

	g := primeGen()
	out := make([]int, 10)
	for i := 0; i < 10; i++ {
		out[i] = g()
	}
	fmt.Println(join(out))
}
