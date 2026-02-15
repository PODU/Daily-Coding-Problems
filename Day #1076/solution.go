// Variadic alternating add/subtract: first arg seeds the total, then the
// rest alternate +, -, +, ... O(n) time, O(1) space.
package main

import "fmt"

func addSubtract(args ...int) int {
	if len(args) == 0 {
		return 0
	}
	result, sign := args[0], 1
	for _, v := range args[1:] {
		result += sign * v
		sign = -sign
	}
	return result
}

func main() {
	fmt.Println("add_subtract(7) =",            addSubtract(7))
	fmt.Println("add_subtract(1)(2)(3) =",      addSubtract(1, 2, 3))
	fmt.Println("add_subtract(-5)(10)(3)(9) =", addSubtract(-5, 10, 3, 9))
}
