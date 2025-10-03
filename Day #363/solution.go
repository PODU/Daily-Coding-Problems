// Day 363: Curried add_subtract that alternately adds/subtracts arguments.
// Go lacks operator-style currying, so a method chain models it: Add returns the
// next link carrying the running value. Time O(k) per chain, Space O(1).
package main

import "fmt"

type AddSubtract struct {
	value int
	count int
}

func addSubtract(first int) AddSubtract { return AddSubtract{first, 1} }

func (a AddSubtract) Add(x int) AddSubtract {
	if a.count%2 == 1 { // arg1 adds, arg2 subtracts, ...
		return AddSubtract{a.value + x, a.count + 1}
	}
	return AddSubtract{a.value - x, a.count + 1}
}

func (a AddSubtract) Val() int { return a.value }

func main() {
	fmt.Println(addSubtract(7).Val())
	fmt.Println("1 + 2 - 3 ->", addSubtract(1).Add(2).Add(3).Val())
	fmt.Println("-5 + 10 - 3 + 9 ->", addSubtract(-5).Add(10).Add(3).Add(9).Val())
}
