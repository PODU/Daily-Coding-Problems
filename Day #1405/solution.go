// Interval DP: T[i][j]/F[i][j] = #ways subexpr of operands i..j is True/False.
// Split at each operator, combine child counts per &,|,^. Time O(n^3), Space O(n^2).
package main

import "fmt"

func countTrue(expr []string) int64 {
	var vals, ops []byte
	for i, e := range expr {
		if i%2 == 0 {
			vals = append(vals, e[0])
		} else {
			ops = append(ops, e[0])
		}
	}
	n := len(vals)
	if n == 0 {
		return 0
	}
	T := make([][]int64, n)
	F := make([][]int64, n)
	for i := range T {
		T[i] = make([]int64, n)
		F[i] = make([]int64, n)
		if vals[i] == 'T' {
			T[i][i] = 1
		} else {
			F[i][i] = 1
		}
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			for k := i; k < j; k++ {
				op := ops[k]
				lt, lf, rt, rf := T[i][k], F[i][k], T[k+1][j], F[k+1][j]
				tot := (lt + lf) * (rt + rf)
				var t int64
				switch op {
				case '&':
					t = lt * rt
				case '|':
					t = lt*rt + lt*rf + lf*rt
				default:
					t = lt*rf + lf*rt
				}
				T[i][j] += t
				F[i][j] += tot - t
			}
		}
	}
	return T[0][n-1]
}

func main() {
	fmt.Println(countTrue([]string{"F", "|", "T", "&", "T"})) // 2
}
