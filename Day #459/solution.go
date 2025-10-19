// Day 459: Fewest perfect squares summing to N.
// Lagrange/Legendre theorem -> answer in {1,2,3,4}, O(sqrt N).
// Reconstruct one decomposition guided by the count.
package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func isSquare(n int) bool {
	if n < 0 {
		return false
	}
	r := int(math.Sqrt(float64(n)))
	for r*r < n {
		r++
	}
	for r*r > n {
		r--
	}
	return r*r == n
}

func minSquares(n int) int {
	if isSquare(n) {
		return 1
	}
	m := n
	for m%4 == 0 {
		m /= 4
	}
	if m%8 == 7 {
		return 4
	}
	for i := 1; i*i <= n; i++ {
		if isSquare(n - i*i) {
			return 2
		}
	}
	return 3
}

func decompose(n int) []int {
	k := minSquares(n)
	res := []int{}
	for k > 0 {
		if k == 1 {
			res = append(res, n)
			break
		}
		for i := int(math.Sqrt(float64(n))); i >= 1; i-- {
			if minSquares(n-i*i) == k-1 {
				res = append(res, i*i)
				n -= i * i
				k--
				break
			}
		}
	}
	return res
}

func main() {
	n := 17
	sq := decompose(n)
	parts := make([]string, len(sq))
	for i, v := range sq {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Printf("%d (%s)\n", minSquares(n), strings.Join(parts, " + ")) // 2 (16 + 1)
}
