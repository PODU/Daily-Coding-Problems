// Job scheduler: call f after n ms using time.AfterFunc (one-shot timer goroutine).
// Time: O(1) to schedule; Space: O(1). Main sleeps so the job runs before exit.
package main

import (
	"fmt"
	"time"
)

func schedule(f func(), n int) {
	time.AfterFunc(time.Duration(n)*time.Millisecond, f)
}

func main() {
	fmt.Println("Scheduling job...")
	schedule(func() { fmt.Println("Job executed after 100 ms") }, 100)
	// Wait long enough for the scheduled job to fire before the program exits.
	time.Sleep(200 * time.Millisecond)
}
