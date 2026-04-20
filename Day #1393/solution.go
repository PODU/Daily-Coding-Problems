// Josephus problem via iterative recurrence j(i)=(j(i-1)+k)%i, answer j(N)+1. O(N) time, O(1) space.
package main

import "fmt"

func josephus(n, k int) int {
	res := 0
	for i := 2; i <= n; i++ {
		res = (res + k) % i
	}
	return res + 1
}

func main() {
	fmt.Println(josephus(5, 2))
}
