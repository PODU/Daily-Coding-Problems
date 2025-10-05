// Day 374: Lowest index i with arr[i]==i in a sorted, distinct-int array.
// f(i)=arr[i]-i is non-decreasing, so binary-search the leftmost i with
// f(i)>=0 and check equality. Time O(log n), Space O(1).
package main

import "fmt"

// returns (index, true) or (0, false) for null.
func fixedPoint(arr []int) (int, bool) {
	lo, hi, ans := 0, len(arr)-1, -1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if arr[mid] >= mid {
			ans = mid
			hi = mid - 1
		} else {
			lo = mid + 1
		}
	}
	if ans != -1 && arr[ans] == ans {
		return ans, true
	}
	return 0, false
}

func main() {
	arr := []int{-5, -3, 2, 3}
	if r, ok := fixedPoint(arr); ok {
		fmt.Println(r) // 2
	} else {
		fmt.Println("null")
	}
}
