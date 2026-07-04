// Day 1762: Count parenthesizations of a boolean expression evaluating to True.
// Interval DP over operands: t[i][j]/f[i][j] = #ways range evals True/False,
// combine across each split operator. Time O(n^3), Space O(n^2).
package main

import "fmt"

func countTrue(tokens []string) int64 {
	var vals, ops []byte
	for i, tk := range tokens {
		if i%2 == 0 {
			vals = append(vals, tk[0])
		} else {
			ops = append(ops, tk[0])
		}
	}
	n := len(vals)
	t := make([][]int64, n)
	f := make([][]int64, n)
	for i := range t {
		t[i] = make([]int64, n)
		f[i] = make([]int64, n)
		if vals[i] == 'T' {
			t[i][i] = 1
		} else {
			f[i][i] = 1
		}
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length-1 < n; i++ {
			j := i + length - 1
			for k := i; k < j; k++ {
				op := ops[k]
				lt, lf, rt, rf := t[i][k], f[i][k], t[k+1][j], f[k+1][j]
				tot := (lt + lf) * (rt + rf)
				switch op {
				case '&':
					t[i][j] += lt * rt
					f[i][j] += tot - lt*rt
				case '|':
					f[i][j] += lf * rf
					t[i][j] += tot - lf*rf
				default: // ^
					t[i][j] += lt*rf + lf*rt
					f[i][j] += lt*rt + lf*rf
				}
			}
		}
	}
	return t[0][n-1]
}

func main() {
	fmt.Println(countTrue([]string{"F", "|", "T", "&", "T"}))
}
