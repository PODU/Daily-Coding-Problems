// Sieve of Eratosthenes: mark multiples of each prime from p*p. O(N log log N) time, O(N) space.
// Bonus: an incremental-sieve generator (goroutine + channel) yielding primes indefinitely.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func sieve(n int) []int {
	if n < 2 {
		return nil
	}
	composite := make([]bool, n)
	primes := []int{}
	for p := 2; p < n; p++ {
		if !composite[p] {
			primes = append(primes, p)
			for m := p * p; m < n; m += p {
				composite[m] = true
			}
		}
	}
	return primes
}

// primeChannel emits primes indefinitely via an incremental sieve.
func primeChannel() <-chan int {
	ch := make(chan int)
	go func() {
		composites := make(map[int][]int)
		candidate := 1
		for {
			candidate++
			if factors, ok := composites[candidate]; ok {
				for _, p := range factors {
					composites[candidate+p] = append(composites[candidate+p], p)
				}
				delete(composites, candidate)
			} else {
				composites[candidate*candidate] = []int{candidate}
				ch <- candidate
			}
		}
	}()
	return ch
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

	ch := primeChannel()
	first10 := make([]int, 10)
	for i := 0; i < 10; i++ {
		first10[i] = <-ch
	}
	fmt.Println(join(first10))
}
