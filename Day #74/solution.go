// Count occurrences of X in N×N table: for each row i (1..N), X appears iff i|X and X/i in [1..N].
// Time O(N), Space O(1).
package main

import "fmt"

func countX(N, X int) int {
	cnt := 0
	for i := 1; i <= N; i++ {
		if X%i == 0 && X/i >= 1 && X/i <= N {
			cnt++
		}
	}
	return cnt
}

func main() {
	fmt.Println(countX(6, 12))
}
