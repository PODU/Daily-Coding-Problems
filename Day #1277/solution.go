// Day 1277: Curried add_subtract — alternately + then - successive args.
// Currying emulated via chained .Next(x); .Value reads the running result. O(1)/call.
package main

import "fmt"

type addSub struct {
	value int
	idx   int // args consumed so far
}

func addSubtract(x int) addSub { return addSub{x, 1} }

func (a addSub) Next(x int) addSub {
	nv := a.value - x
	if a.idx%2 == 1 {
		nv = a.value + x
	}
	return addSub{nv, a.idx + 1}
}

func (a addSub) Value() int { return a.value }

func main() {
	fmt.Println(addSubtract(7).Value())                          // 7
	fmt.Println(addSubtract(1).Next(2).Next(3).Value())          // 0
	fmt.Println(addSubtract(-5).Next(10).Next(3).Next(9).Value()) // 11
}
