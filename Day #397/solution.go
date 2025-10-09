// Greedy activity selection: sort by end time, pick job if start >= last end (intervals [start,end)).
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

type Job struct{ start, end int }

func selectJobs(jobs []Job) []Job {
	sorted := make([]Job, len(jobs))
	copy(sorted, jobs)
	sort.Slice(sorted, func(i, j int) bool { return sorted[i].end < sorted[j].end })
	chosen := []Job{}
	lastEnd := math.MinInt64
	for _, j := range sorted {
		if j.start >= lastEnd {
			chosen = append(chosen, j)
			lastEnd = j.end
		}
	}
	return chosen
}

func main() {
	jobs := []Job{{0, 6}, {1, 4}, {3, 5}, {3, 8}, {4, 7}, {5, 9}, {6, 10}, {8, 11}}
	chosen := selectJobs(jobs)
	parts := make([]string, len(chosen))
	for i, j := range chosen {
		parts[i] = fmt.Sprintf("(%d, %d)", j.start, j.end)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
