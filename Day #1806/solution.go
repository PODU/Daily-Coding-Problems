// Tower of Hanoi: recursive divide-and-conquer.
// Time: O(2^n) moves (optimal/minimal). Space: O(n) recursion depth.
package main

import "fmt"

func hanoi(n, from, to, via int) {
	if n == 0 {
		return
	}
	hanoi(n-1, from, via, to)
	fmt.Printf("Move %d to %d\n", from, to)
	hanoi(n-1, via, to, from)
}

func main() {
	hanoi(3, 1, 3, 2)
}
