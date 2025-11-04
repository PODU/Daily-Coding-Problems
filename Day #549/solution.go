// Single number among triples: sum each bit position mod 3 to rebuild the unique value. O(N) time, O(1) space.
package main

import "fmt"

func singleNumber(nums []int) int {
	result := 0
	for b := 0; b < 32; b++ {
		cnt := 0
		for _, x := range nums {
			cnt += (x >> b) & 1
		}
		if cnt%3 != 0 {
			result |= 1 << b
		}
	}
	return result
}

func main() {
	fmt.Println(singleNumber([]int{6, 1, 3, 3, 3, 6, 6}))
	fmt.Println(singleNumber([]int{13, 19, 13, 13}))
}
