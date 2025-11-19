// Combination lock: BFS over 1000 states from "000" to target, avoiding deadends.
// Each of 3 wheels has 2 neighbors (+1/-1 mod 10) => 6 neighbors. Time/space O(1000).
package main

import "fmt"

func openLock(deadends []string, target, start string) int {
	dead := make(map[string]bool)
	for _, d := range deadends {
		dead[d] = true
	}
	if dead[start] {
		return -1
	}
	if start == target {
		return 0
	}
	visited := map[string]bool{start: true}
	frontier := []string{start}
	steps := 0
	for len(frontier) > 0 {
		steps++
		var next []string
		for _, cur := range frontier {
			for w := 0; w < 3; w++ {
				for _, d := range []int{1, 9} { // +1 or -1 (mod 10)
					b := []byte(cur)
					b[w] = byte('0' + (int(b[w]-'0')+d)%10)
					nxt := string(b)
					if dead[nxt] || visited[nxt] {
						continue
					}
					if nxt == target {
						return steps
					}
					visited[nxt] = true
					next = append(next, nxt)
				}
			}
		}
		frontier = next
	}
	return -1
}

func main() {
	r1 := openLock([]string{"010", "021"}, "020", "000")
	fmt.Println("Min moves to open lock (target 020):", r1)

	r2 := openLock([]string{"001", "010", "100", "009", "090", "900"}, "111", "000")
	if r2 == -1 {
		fmt.Println("Impossible case (target 111): None")
	} else {
		fmt.Println("Impossible case (target 111):", r2)
	}
}
