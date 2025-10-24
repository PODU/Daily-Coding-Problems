// Day 483: Josephus problem.
// Iterative recurrence f(i)=(f(i-1)+k)%i in O(n) time, O(1) space.
// Special O(log n) closed form when k=2.
package main

import "fmt"

func josephus(n, k int) int {
	result := 0 // 0-indexed survivor among 1 person
	for i := 2; i <= n; i++ {
		result = (result + k) % i
	}
	return result + 1 // 1-indexed
}

// O(log n) special case for k == 2.
func josephusK2(n int) int {
	p := 1
	for p*2 <= n {
		p *= 2
	}
	return 2*(n-p) + 1
}

func main() {
	n, k := 5, 2
	fmt.Println(josephus(n, k)) // 3
}
