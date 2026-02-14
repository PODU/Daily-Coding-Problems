// Gale-Shapley men-proposing stable matching. Time: O(N^2), Space: O(N^2).
package main

import (
	"fmt"
	"sort"
)

func main() {
	men   := []string{"andrew", "bill", "chester"}
	women := []string{"abigail", "betty", "caroline"}
	wi := map[string]int{}
	mi := map[string]int{}
	for i, w := range women { wi[w] = i }
	for i, m := range men   { mi[m] = i }

	gp := [][]int{
		{wi["caroline"], wi["abigail"], wi["betty"]},
		{wi["caroline"], wi["betty"],   wi["abigail"]},
		{wi["betty"],    wi["caroline"],wi["abigail"]},
	}
	// gr[w][m] = preference rank of man m for woman w
	gr := [3][3]int{}
	gr[wi["abigail"]][mi["andrew"]]=0; gr[wi["abigail"]][mi["bill"]]=1; gr[wi["abigail"]][mi["chester"]]=2
	gr[wi["betty"]][mi["bill"]]=0;     gr[wi["betty"]][mi["andrew"]]=1; gr[wi["betty"]][mi["chester"]]=2
	gr[wi["caroline"]][mi["bill"]]=0;  gr[wi["caroline"]][mi["chester"]]=1; gr[wi["caroline"]][mi["andrew"]]=2

	wp  := [3]int{-1, -1, -1}
	nxt := [3]int{}
	q := []int{0, 1, 2}
	for len(q) > 0 {
		m := q[0]; q = q[1:]
		w := gp[m][nxt[m]]; nxt[m]++
		if wp[w] == -1 {
			wp[w] = m
		} else if gr[w][m] < gr[w][wp[w]] {
			q = append(q, wp[w]); wp[w] = m
		} else {
			q = append(q, m)
		}
	}

	type pair struct{ man, woman string }
	res := []pair{}
	for w := 0; w < 3; w++ {
		res = append(res, pair{men[wp[w]], women[w]})
	}
	sort.Slice(res, func(i, j int) bool { return res[i].man < res[j].man })
	for _, p := range res {
		fmt.Printf("%s -> %s\n", p.man, p.woman)
	}
}
