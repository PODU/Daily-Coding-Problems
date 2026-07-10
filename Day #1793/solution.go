// Collect positions p_i of people, set q_i = p_i - i, answer = sum |q_i - median(q)|.
// Time O(n), Space O(m).
package main

import "fmt"

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func minCost(seats []int) int {
	q := []int{}
	i := 0
	for j, s := range seats {
		if s == 1 {
			q = append(q, j-i)
			i++
		}
	}
	if len(q) == 0 {
		return 0
	}
	med := q[len(q)/2]
	total := 0
	for _, v := range q {
		total += abs(v - med)
	}
	return total
}

func main() {
	seats := []int{0, 1, 1, 0, 1, 0, 0, 0, 1}
	fmt.Println(minCost(seats)) // expected 5
}
