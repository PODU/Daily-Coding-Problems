// Dutch National Flag 3-way partition (R<G<B). In-place, O(n) time, O(1) space, swaps only.
package main

import (
	"fmt"
	"strings"
)

func sortRGB(a []byte) {
	low, mid, high := 0, 0, len(a)-1
	for mid <= high {
		switch a[mid] {
		case 'R':
			a[low], a[mid] = a[mid], a[low]
			low++
			mid++
		case 'G':
			mid++
		default:
			a[mid], a[high] = a[high], a[mid]
			high--
		}
	}
}

func main() {
	a := []byte{'G', 'B', 'R', 'R', 'B', 'R', 'G'}
	sortRGB(a)
	parts := make([]string, len(a))
	for i, c := range a {
		parts[i] = "'" + string(c) + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
