// Day 371: Solve a system of addition-only equations over variables/constants.
// Build a linear system A x = b and run Gauss-Jordan elimination; unique integer
// solution -> mapping, otherwise null. Time O(eqs * vars^2).
package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func isNumber(t string) bool {
	if t == "" {
		return false
	}
	i := 0
	if t[0] == '-' {
		i = 1
	}
	if i == len(t) {
		return false
	}
	for ; i < len(t); i++ {
		if t[i] < '0' || t[i] > '9' {
			return false
		}
	}
	return true
}

func solve(text string) (map[string]int, bool) {
	type eq struct {
		coeffs map[string]float64
		b      float64
	}
	var eqs []eq
	varset := map[string]bool{}
	for _, raw := range strings.Split(text, "\n") {
		line := strings.TrimSpace(raw)
		if line == "" {
			continue
		}
		sides := strings.SplitN(line, "=", 2)
		coeffs := map[string]float64{}
		b := 0.0
		for _, tok := range strings.Split(sides[0], "+") {
			t := strings.TrimSpace(tok)
			if isNumber(t) {
				v, _ := strconv.Atoi(t)
				b -= float64(v)
			} else {
				coeffs[t]++
				varset[t] = true
			}
		}
		for _, tok := range strings.Split(sides[1], "+") {
			t := strings.TrimSpace(tok)
			if isNumber(t) {
				v, _ := strconv.Atoi(t)
				b += float64(v)
			} else {
				coeffs[t]--
				varset[t] = true
			}
		}
		eqs = append(eqs, eq{coeffs, b})
	}

	var vars []string
	for v := range varset {
		vars = append(vars, v)
	}
	sort.Strings(vars)
	idx := map[string]int{}
	for i, v := range vars {
		idx[v] = i
	}
	n := len(vars)
	aug := make([][]float64, len(eqs))
	for r, e := range eqs {
		aug[r] = make([]float64, n+1)
		for v, c := range e.coeffs {
			aug[r][idx[v]] += c
		}
		aug[r][n] = e.b
	}

	m := len(aug)
	var pivotCols []int
	pr := 0
	for col := 0; col < n; col++ {
		sel := -1
		for r := pr; r < m; r++ {
			if math.Abs(aug[r][col]) > 1e-9 {
				sel = r
				break
			}
		}
		if sel == -1 {
			continue
		}
		aug[pr], aug[sel] = aug[sel], aug[pr]
		pv := aug[pr][col]
		for c := 0; c <= n; c++ {
			aug[pr][c] /= pv
		}
		for r := 0; r < m; r++ {
			if r != pr && math.Abs(aug[r][col]) > 1e-9 {
				f := aug[r][col]
				for c := 0; c <= n; c++ {
					aug[r][c] -= f * aug[pr][c]
				}
			}
		}
		pivotCols = append(pivotCols, col)
		pr++
	}
	for r := 0; r < m; r++ {
		allZero := true
		for c := 0; c < n; c++ {
			if math.Abs(aug[r][c]) > 1e-9 {
				allZero = false
			}
		}
		if allZero && math.Abs(aug[r][n]) > 1e-9 {
			return nil, false
		}
	}
	if len(pivotCols) < n {
		return nil, false
	}
	sol := map[string]int{}
	for i, col := range pivotCols {
		sol[vars[col]] = int(math.Round(aug[i][n]))
	}
	return sol, true
}

func main() {
	text := "y = x + 1\n5 = x + 3\n10 = z + y + 2"
	sol, ok := solve(text)
	if !ok {
		fmt.Println("null")
		return
	}
	keys := make([]string, 0, len(sol))
	for k := range sol {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	var sb strings.Builder
	sb.WriteString("{\n")
	for i, k := range keys {
		if i > 0 {
			sb.WriteString(",\n")
		}
		sb.WriteString(fmt.Sprintf("  %s: %d", k, sol[k]))
	}
	sb.WriteString("\n}")
	fmt.Println(sb.String())
}
