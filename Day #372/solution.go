// Day 372: Count digits of a natural number without loops.
// Recursion: 1 digit for n<10, else 1 + digits(n/10). Time O(d), Space O(d).
package main

import "fmt"

func numDigits(n int64) int {
	if n < 10 {
		return 1
	}
	return 1 + numDigits(n/10)
}

func main() {
	fmt.Println(numDigits(0))     // 1
	fmt.Println(numDigits(7))     // 1
	fmt.Println(numDigits(42))    // 2
	fmt.Println(numDigits(12345)) // 5
}
