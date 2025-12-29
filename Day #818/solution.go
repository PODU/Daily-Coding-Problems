// Sieve of Eratosthenes for primes below N; incremental generator via goroutine channel + trial division.
// Sieve: O(N log log N). Generator yields primes indefinitely.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

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

// Unbounded prime generator over a channel, trial division by found primes up to sqrt.
func genPrimes() <-chan int {
	ch := make(chan int)
	go func() {
		var primes []int
		cand := 2
		for {
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
				ch <- cand
			}
			cand++
		}
	}()
	return ch
}

func firstN(ch <-chan int, n int) []int {
	out := make([]int, 0, n)
	for i := 0; i < n; i++ {
		out = append(out, <-ch)
	}
	return out
}

func format(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = strconv.Itoa(x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println("Primes below 30: " + format(sieve(30)))
	fmt.Println("First 10 primes: " + format(firstN(genPrimes(), 10)))
}
