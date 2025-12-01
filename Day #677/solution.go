// Sieve of Eratosthenes for primes < N (O(N log log N)); plus an incremental
// generator using a map of next composite multiples. Space: O(N) sieve.
package main

import (
	"fmt"
	"strings"
)

func sieve(n int) []int {
	size := n
	if size < 0 {
		size = 0
	}
	isComp := make([]bool, size)
	primes := []int{}
	for i := 2; i < n; i++ {
		if !isComp[i] {
			primes = append(primes, i)
			for j := i * i; j < n; j += i {
				isComp[j] = true
			}
		}
	}
	return primes
}

// PrimeGen yields primes one-by-one via a map of next composites.
type PrimeGen struct {
	composites map[int]int
	current    int
}

func NewPrimeGen() *PrimeGen {
	return &PrimeGen{composites: make(map[int]int), current: 1}
}

func (g *PrimeGen) Next() int {
	for {
		g.current++
		if prime, ok := g.composites[g.current]; !ok {
			g.composites[g.current*g.current] = g.current // prime found
			return g.current
		} else {
			delete(g.composites, g.current)
			x := g.current + prime
			for {
				if _, exists := g.composites[x]; !exists {
					break
				}
				x += prime
			}
			g.composites[x] = prime
		}
	}
}

func toList(nums []int) string {
	parts := make([]string, len(nums))
	for i, v := range nums {
		parts[i] = fmt.Sprintf("%d", v)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(toList(sieve(100)))
	gen := NewPrimeGen()
	first10 := make([]int, 10)
	for i := 0; i < 10; i++ {
		first10[i] = gen.Next()
	}
	fmt.Println(toList(first10))
}
