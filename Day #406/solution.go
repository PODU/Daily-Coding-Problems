// Boolean parenthesization: count ways the expression evaluates to True.
// Interval DP storing #True/#False per substring. Time O(n^3), Space O(n^2).
package main

import "fmt"

func countTrue(a []string) int64 {
	var val, op []byte
	for i := 0; i < len(a); i++ {
		if i%2 == 0 {
			val = append(val, a[i][0])
		} else {
			op = append(op, a[i][0])
		}
	}
	M := len(val)
	T := make([][]int64, M)
	F := make([][]int64, M)
	for i := range T {
		T[i] = make([]int64, M)
		F[i] = make([]int64, M)
		if val[i] == 'T' {
			T[i][i] = 1
		} else {
			F[i][i] = 1
		}
	}
	for length := 2; length <= M; length++ {
		for i := 0; i+length-1 < M; i++ {
			j := i + length - 1
			for k := i; k < j; k++ {
				o := op[k]
				lt, lf, rt, rf := T[i][k], F[i][k], T[k+1][j], F[k+1][j]
				tot := (lt + lf) * (rt + rf)
				switch o {
				case '&':
					T[i][j] += lt * rt
					F[i][j] += tot - lt*rt
				case '|':
					T[i][j] += tot - lf*rf
					F[i][j] += lf * rf
				default:
					T[i][j] += lt*rf + lf*rt
					F[i][j] += lt*rt + lf*rf
				}
			}
		}
	}
	return T[0][M-1]
}

func main() {
	fmt.Println(countTrue([]string{"F", "|", "T", "&", "T"}))
}
