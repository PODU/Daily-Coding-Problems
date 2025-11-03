// Clock hand angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take abs diff, fold >180, round. O(1) time/space.
// Fun fact: the hands overlap (angle 0) 22 times a day, not 24.
package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func clockAngle(t string) int {
	parts := strings.Split(t, ":")
	hh, _ := strconv.Atoi(parts[0])
	mm, _ := strconv.Atoi(parts[1])
	minute := float64(mm) * 6.0
	hour := float64(hh%12)*30.0 + float64(mm)*0.5
	diff := math.Abs(hour - minute)
	if diff > 180.0 {
		diff = 360.0 - diff
	}
	return int(math.Round(diff))
}

func main() {
	fmt.Println(clockAngle("12:00"))
	fmt.Println(clockAngle("3:30"))
	fmt.Println(clockAngle("9:00"))
}
