// add_subtract: chainable builder. Add(x) alternates +/- ; Value() reads result.
// O(n) time over number of args, O(1) extra space.
package main

import "fmt"

type AddSub struct {
	total int64
	sign  int64 // applied to next argument
}

func add_subtract(first int64) AddSub { return AddSub{first, 1} }

func (a AddSub) Add(x int64) AddSub { return AddSub{a.total + a.sign*x, -a.sign} }
func (a AddSub) Value() int64       { return a.total }

func main() {
	fmt.Println(add_subtract(7).Value())                       // 7
	fmt.Println(add_subtract(1).Add(2).Add(3).Value())         // 0
	fmt.Println(add_subtract(-5).Add(10).Add(3).Add(9).Value()) // 11
}
