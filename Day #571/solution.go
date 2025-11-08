// Tower of Hanoi: classic recursion. Move n disks from rod `from` to `to` using `via`.
// Prints 2^n - 1 moves. O(2^n) time, O(n) recursion depth.
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
