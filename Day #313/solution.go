// Day 313: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends.
// BFS over <=1000 states. O(1000) time. Returns -1 to represent None.
package main

import "fmt"

func openLock(target string, deadends []string) int {
	dead := map[string]bool{}
	for _, d := range deadends {
		dead[d] = true
	}
	start := "000"
	if dead[start] {
		return -1
	}
	if start == target {
		return 0
	}
	dist := map[string]int{start: 0}
	q := []string{start}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for i := 0; i < 3; i++ {
			for _, d := range []int{-1, 1} {
				b := []byte(cur)
				b[i] = byte('0' + ((int(b[i]-'0')+d+10)%10))
				nx := string(b)
				if dead[nx] {
					continue
				}
				if _, ok := dist[nx]; ok {
					continue
				}
				dist[nx] = dist[cur] + 1
				if nx == target {
					return dist[nx]
				}
				q = append(q, nx)
			}
		}
	}
	return -1
}

func main() {
	fmt.Println(openLock("123", []string{})) // 6
}
