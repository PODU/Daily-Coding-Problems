// Curried add_subtract: each call alternates +/- on the running total.
// Go has no call operator, so we expose a fluent .Apply(...) chain.
// Time: O(1) per call. Space: O(1).
package main

import "fmt"

type AddSub struct {
	total int
	sign  int // sign applied to the NEXT argument
}

func (a AddSub) Apply(y int) AddSub { return AddSub{a.total + a.sign*y, -a.sign} }
func (a AddSub) Value() int         { return a.total }

func addSubtract(x int) AddSub { return AddSub{x, 1} }

func main() {
	fmt.Println(addSubtract(7).Value())                     // 7
	fmt.Println(addSubtract(1).Apply(2).Apply(3).Value())   // 0
	fmt.Println(addSubtract(-5).Apply(10).Apply(3).Apply(9).Value()) // 11
}
