// Lazy bartender = min set cover (NP-hard). Greedy: repeatedly pick drink covering
// most uncovered customers. Time O(D^2*C), Space O(D*C).
package main

import "fmt"

func main() {
	preferences := map[int][]int{
		0: {0, 1, 3, 6}, 1: {1, 4, 7}, 2: {2, 4, 7, 5}, 3: {3, 2, 5}, 4: {5, 8},
	}
	drinkToCust := map[int]map[int]bool{}
	uncovered := map[int]bool{}
	for cust, drinks := range preferences {
		uncovered[cust] = true
		for _, d := range drinks {
			if drinkToCust[d] == nil {
				drinkToCust[d] = map[int]bool{}
			}
			drinkToCust[d][cust] = true
		}
	}
	learned := 0
	for len(uncovered) > 0 {
		bestDrink, bestCount := -1, 0
		for drink, custs := range drinkToCust {
			cnt := 0
			for c := range custs {
				if uncovered[c] {
					cnt++
				}
			}
			if cnt > bestCount {
				bestCount, bestDrink = cnt, drink
			}
		}
		if bestDrink == -1 {
			break
		}
		for c := range drinkToCust[bestDrink] {
			delete(uncovered, c)
		}
		learned++
	}
	fmt.Println(learned)
}
