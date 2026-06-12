// Bijective base-26: while n>0, n--, prepend 'A'+(n%26), n/=26. O(log n) time, O(log n) space.
package main

import "fmt"

func columnTitle(n int) string {
	b := []byte{}
	for n > 0 {
		n--
		b = append(b, byte('A'+(n%26)))
		n /= 26
	}
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		b[i], b[j] = b[j], b[i]
	}
	return string(b)
}

func main() {
	fmt.Println(columnTitle(1))
	fmt.Println(columnTitle(27))
}
