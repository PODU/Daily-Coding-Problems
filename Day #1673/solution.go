// Day 1673: Min wheel rotations from "000" to target avoiding dead ends.
// BFS over <=1000 states, each with 6 neighbors. Time O(1000), Space O(1000).
package main

import "fmt"

func openLock(target string, deadends []string) (int, bool) {
	dead := map[string]bool{}
	for _, d := range deadends {
		dead[d] = true
	}
	start := "000"
	if dead[start] || dead[target] {
		return 0, false
	}
	if start == target {
		return 0, true
	}
	dist := map[string]int{start: 0}
	q := []string{start}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for i := 0; i < 3; i++ {
			for _, d := range []int{-1, 1} {
				digit := (int(cur[i]-'0') + d + 10) % 10
				b := []byte(cur)
				b[i] = byte('0' + digit)
				nxt := string(b)
				if dead[nxt] {
					continue
				}
				if _, ok := dist[nxt]; ok {
					continue
				}
				dist[nxt] = dist[cur] + 1
				if nxt == target {
					return dist[nxt], true
				}
				q = append(q, nxt)
			}
		}
	}
	return 0, false
}

func main() {
	if r, ok := openLock("345", []string{"333"}); ok {
		fmt.Println(r) // 12
	} else {
		fmt.Println("None")
	}
}
