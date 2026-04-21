// Single scan: count drops; on a drop, greedily fix by lowering a[i] or
// raising a[i+1] depending on a[i-1]. >1 drop => false. Time O(n), Space O(1).
package main

import "fmt"

func checkPossibility(src []int) bool {
	a := append([]int(nil), src...)
	cnt := 0
	for i := 0; i+1 < len(a); i++ {
		if a[i] > a[i+1] {
			cnt++
			if cnt > 1 {
				return false
			}
			if i == 0 || a[i-1] <= a[i+1] {
				a[i] = a[i+1]
			} else {
				a[i+1] = a[i]
			}
		}
	}
	return true
}

func main() {
	fmt.Println(checkPossibility([]int{10, 5, 7}))
	fmt.Println(checkPossibility([]int{10, 5, 1}))
}
