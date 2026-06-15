// Iterate counting integers; pick those whose digit sum == 10. O(answer) time, O(1) space.
package main

import "fmt"

func digitSum(x int) int { s := 0; for x > 0 { s += x % 10; x /= 10 }; return s }
func nthPerfect(n int) int { x, c := 0, 0; for c < n { x++; if digitSum(x) == 10 { c++ } }; return x }
func main() { fmt.Println(nthPerfect(1)); fmt.Println(nthPerfect(2)) }
