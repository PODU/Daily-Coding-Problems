// Day 1113 - Validate an American-style crossword grid ('#' black, '.' white)
// Approach: 180-degree symmetry, every white cell in across & down run >= 3,
// and white connectivity. Time: O(N^2), Space: O(N^2).
package main

import "fmt"

func isCrossword(grid []string) bool {
	n := len(grid)
	if n == 0 {
		return false
	}
	for _, r := range grid {
		if len(r) != n {
			return false
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if (grid[i][j] == '#') != (grid[n-1-i][n-1-j] == '#') {
				return false
			}
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == '.' {
				length, k := 1, j-1
				for k >= 0 && grid[i][k] == '.' {
					length++
					k--
				}
				k = j + 1
				for k < n && grid[i][k] == '.' {
					length++
					k++
				}
				if length < 3 {
					return false
				}
				length, k = 1, i-1
				for k >= 0 && grid[k][j] == '.' {
					length++
					k--
				}
				k = i + 1
				for k < n && grid[k][j] == '.' {
					length++
					k++
				}
				if length < 3 {
					return false
				}
			}
		}
	}

	type pt struct{ i, j int }
	var whites []pt
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == '.' {
				whites = append(whites, pt{i, j})
			}
		}
	}
	if len(whites) == 0 {
		return true
	}

	seen := make([][]bool, n)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	st := []pt{whites[0]}
	seen[whites[0].i][whites[0].j] = true
	cnt := 1
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(st) > 0 {
		c := st[len(st)-1]
		st = st[:len(st)-1]
		for _, d := range dirs {
			ni, nj := c.i+d[0], c.j+d[1]
			if ni >= 0 && ni < n && nj >= 0 && nj < n && grid[ni][nj] == '.' && !seen[ni][nj] {
				seen[ni][nj] = true
				cnt++
				st = append(st, pt{ni, nj})
			}
		}
	}
	return cnt == len(whites)
}

func main() {
	valid := []string{"...##", ".....", ".....", ".....", "##..."}
	invalid := []string{".....", ".....", ".....", ".....", "....#"}
	b2s := func(b bool) string {
		if b {
			return "True"
		}
		return "False"
	}
	fmt.Println("Grid 1 valid:", b2s(isCrossword(valid)))
	fmt.Println("Grid 2 valid:", b2s(isCrossword(invalid)))
}
