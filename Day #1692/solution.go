// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
// Bonus: hands overlap (angle 0) 11 times per 12 hours, at t = (12/11)*k hours for k=0..10.
package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func clockAngle(hh, mm int) int {
	minuteAngle := float64(mm) * 6.0
	hourAngle := float64(hh%12)*30.0 + float64(mm)*0.5
	diff := math.Abs(hourAngle - minuteAngle)
	angle := math.Min(diff, 360.0-diff)
	return int(math.Round(angle))
}

func main() {
	t := "3:15"
	parts := strings.Split(t, ":")
	hh, _ := strconv.Atoi(strings.TrimSpace(parts[0]))
	mm, _ := strconv.Atoi(strings.TrimSpace(parts[1]))
	fmt.Println(clockAngle(hh, mm))
}
