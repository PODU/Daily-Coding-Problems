// Day 1499: Job scheduler that calls f after n ms using time.AfterFunc.
// A channel signals completion so main waits. Time O(1), Space O(1).
package main

import (
	"fmt"
	"time"
)

func schedule(f func(), n int) *time.Timer {
	return time.AfterFunc(time.Duration(n)*time.Millisecond, f)
}

func main() {
	done := make(chan struct{})
	schedule(func() {
		fmt.Println("Job executed after delay")
		close(done)
	}, 100)
	<-done // wait for the scheduled job before exiting
}
