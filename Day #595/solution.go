// Kaprekar's routine: repeatedly subtract ascending-digit number from
// descending-digit number (4-digit, zero-padded) until reaching 6174.
// Bounded to <=7 steps. Time O(1), Space O(1).
package main

import (
	"fmt"
	"sort"
	"strconv"
)

func main() {
	n := 1234
	steps := 0
	for n != 6174 {
		s := fmt.Sprintf("%04d", n)
		digits := []byte(s)
		sort.Slice(digits, func(i, j int) bool { return digits[i] < digits[j] })
		asc, _ := strconv.Atoi(string(digits))
		// reverse for descending
		desc := make([]byte, len(digits))
		for i := range digits {
			desc[i] = digits[len(digits)-1-i]
		}
		hi, _ := strconv.Atoi(string(desc))
		n = hi - asc
		steps++
		fmt.Printf("%04d - %04d = %04d\n", hi, asc, n)
	}
	fmt.Printf("Steps: %d\n", steps)
}
