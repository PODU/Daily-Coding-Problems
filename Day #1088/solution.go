// Boolean parenthesization via interval DP counting True/False groupings. Time O(n^3), Space O(n^2).
package main

import "fmt"

func countTrue(expr []byte) int64 {
	m := len(expr)
	n := (m + 1) / 2
	val := make([]bool, n)
	ops := make([]byte, 0, n)
	for i := 0; i < m; i++ {
		if i%2 == 0 {
			val[i/2] = expr[i] == 'T'
		} else {
			ops = append(ops, expr[i])
		}
	}
	T := make([][]int64, n)
	F := make([][]int64, n)
	for i := range T {
		T[i] = make([]int64, n)
		F[i] = make([]int64, n)
	}
	for i := 0; i < n; i++ {
		if val[i] {
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
				switch op {
				case '&':
					T[i][j] += lt * rt
					F[i][j] += lf*rf + lf*rt + lt*rf
				case '|':
					T[i][j] += lt*rt + lt*rf + lf*rt
					F[i][j] += lf * rf
				default:
					T[i][j] += lt*rf + lf*rt
					F[i][j] += lt*rt + lf*rf
				}
			}
		}
	}
	return T[0][n-1]
}

func main() {
	expr := []byte{'F', '|', 'T', '&', 'T'}
	fmt.Println(countTrue(expr))
}
