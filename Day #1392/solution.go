// Circular lock min moves via BFS over 1000 states; each state has 6 neighbors (3 wheels x +/-1 mod 10). O(states) time/space.
package main

import "fmt"

func minMoves(target string, deadends []string) int {
	dead := make(map[string]bool)
	for _, d := range deadends {
		dead[d] = true
	}
	if dead["000"] || dead[target] {
		return -1
	}
	if target == "000" {
		return 0
	}
	visited := map[string]bool{"000": true}
	type item struct {
		s string
		d int
	}
	queue := []item{{"000", 0}}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		b := []byte(cur.s)
		for i := 0; i < 3; i++ {
			for _, delta := range []int{1, 9} {
				nb := make([]byte, 3)
				copy(nb, b)
				nb[i] = byte('0' + (int(b[i]-'0')+delta)%10)
				nx := string(nb)
				if visited[nx] || dead[nx] {
					continue
				}
				if nx == target {
					return cur.d + 1
				}
				visited[nx] = true
				queue = append(queue, item{nx, cur.d + 1})
			}
		}
	}
	return -1
}

func main() {
	deadends := []string{"100", "020", "001"}
	fmt.Println(minMoves("333", deadends))
}
