// Day 938: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends. BFS over
// 1000 states, each with 6 neighbors. Time O(1000*6), Space O(1000). Returns -1 (None) if blocked.
package main

import "fmt"

func minMoves(target string, deadList []string) int {
	dead := make(map[string]bool)
	for _, s := range deadList {
		dead[s] = true
	}
	start := "000"
	if dead[start] || dead[target] {
		return -1
	}
	if start == target {
		return 0
	}
	visited := map[string]bool{start: true}
	queue := []string{start}
	depth := 0
	for len(queue) > 0 {
		next := []string{}
		depth++
		for _, cur := range queue {
			for i := 0; i < 3; i++ {
				for _, delta := range []int{1, 9} {
					b := []byte(cur)
					b[i] = byte('0' + (int(cur[i]-'0')+delta)%10)
					ns := string(b)
					if dead[ns] || visited[ns] {
						continue
					}
					if ns == target {
						return depth
					}
					visited[ns] = true
					next = append(next, ns)
				}
			}
		}
		queue = next
	}
	return -1
}

func main() {
	fmt.Println(minMoves("123", []string{})) // 6
	dead := []string{"100", "900", "010", "090", "001", "009"}
	r := minMoves("111", dead)
	if r == -1 {
		fmt.Println("None")
	} else {
		fmt.Println(r)
	}
	// None
}
