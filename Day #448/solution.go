// Day 448: Dutch National Flag sort of R/G/B. O(n) time, O(1) space, in-place
// with three pointers (low=R boundary, high=B boundary, mid=scanner).
package main

import "fmt"

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
		default: // 'B'
			a[mid], a[high] = a[high], a[mid]
			high--
		}
	}
}

func main() {
	a := []byte{'G', 'B', 'R', 'R', 'B', 'R', 'G'}
	sortRGB(a)
	fmt.Print("[")
	for i, c := range a {
		fmt.Printf("'%c'", c)
		if i+1 < len(a) {
			fmt.Print(", ")
		}
	}
	fmt.Println("]")
	// ['R', 'R', 'R', 'G', 'G', 'B', 'B']
}
