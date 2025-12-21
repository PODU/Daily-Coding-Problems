// Day 776: Josephus problem. General O(N) recurrence j=(j+k)%i.
// For k=2 an O(log N) closed form is also given. Returns 1-indexed survivor.
package main

import "fmt"

func josephus(n, k int) int {
	r := 0
	for i := 2; i <= n; i++ {
		r = (r + k) % i
	}
	return r + 1
}

func josephusK2(n int) int {
	p := 1
	for p*2 <= n {
		p *= 2
	}
	return 2*(n-p) + 1
}

func main() {
	fmt.Println(josephus(5, 2)) // 3
	fmt.Println(josephusK2(5))  // 3
}
