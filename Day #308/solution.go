// Day 308: Count parenthesizations evaluating True. Interval DP. O(n^3) time, O(n^2) space.
package main

import "fmt"

func countTrue(e []string) int64 {
	n := len(e)
	V := (n + 1) / 2
	T := make([][]int64, V)
	F := make([][]int64, V)
	for i := range T {
		T[i] = make([]int64, V)
		F[i] = make([]int64, V)
	}
	for i := 0; i < V; i++ {
		if e[2*i] == "T" {
			T[i][i] = 1
		} else {
			F[i][i] = 1
		}
	}
	for length := 2; length <= V; length++ {
		for i := 0; i+length-1 < V; i++ {
			j := i + length - 1
			for k := i; k < j; k++ {
				op := e[2*k+1]
				lt, lf := T[i][k], F[i][k]
				rt, rf := T[k+1][j], F[k+1][j]
				total := (lt + lf) * (rt + rf)
				var t int64
				switch op {
				case "&":
					t = lt * rt
				case "|":
					t = lt*rt + lt*rf + lf*rt
				default: // ^
					t = lt*rf + lf*rt
				}
				T[i][j] += t
				F[i][j] += total - t
			}
		}
	}
	return T[0][V-1]
}

func main() {
	fmt.Println(countTrue([]string{"F", "|", "T", "&", "T"})) // 2
}
