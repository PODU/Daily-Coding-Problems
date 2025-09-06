// Day 225: Josephus problem - position (1-indexed) of last survivor.
// Approach: general O(N) recurrence J(i)=(J(i-1)+k)%i. Bonus: k==2 closed form O(log N): 2*(n-2^floor(log2 n))+1.
package main

import "fmt"

func josephus(n, k int) int {
	if k == 2 {
		p := 1
		for p*2 <= n {
			p *= 2 // highest power of 2 <= n
		}
		return 2*(n-p) + 1
	}
	res := 0 // 0-indexed
	for i := 2; i <= n; i++ {
		res = (res + k) % i
	}
	return res + 1
}

func main() {
	fmt.Println(josephus(5, 2)) // 3
}
