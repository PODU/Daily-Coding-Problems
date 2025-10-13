// Day 424: Two unique elements via XOR partition. O(n) time, O(1) space.
// XOR all -> a^b; isolate a low set bit; partition & XOR each group -> a, b.
package main

import "fmt"

func main() {
	a := []int{2, 4, 6, 8, 10, 2, 6, 10}
	x := 0
	for _, v := range a {
		x ^= v
	}
	bit := x & (-x)
	p, q := 0, 0
	for _, v := range a {
		if v&bit != 0 {
			p ^= v
		} else {
			q ^= v
		}
	}
	if p > q {
		p, q = q, p
	}
	fmt.Printf("%d and %d\n", p, q)
}
