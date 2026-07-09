// Josephus problem: iterative recurrence J(n)=(J(n-1)+k)%n, plus k=2 closed form.
// Time O(N) (O(log N) for k=2 closed form), Space O(1).
package main

import "fmt"

func josephus(n, k int) int {
	r := 0
	for i := 2; i <= n; i++ {
		r = (r + k) % i
	}
	return r + 1
}

func josephus2(n int) int { // k==2 closed form
	p := 1
	for p*2 <= n {
		p *= 2
	}
	return 2*(n-p) + 1
}

func main() {
	n, k := 5, 2
	ans := josephus(n, k)
	if k == 2 {
		ans = josephus2(n)
	}
	fmt.Println(ans)
}
