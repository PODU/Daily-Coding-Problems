// Job scheduler: run f after n ms using time.AfterFunc (background timer goroutine).
// Schedule is O(1); main waits on a channel signaled by the job before exiting.
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
		fmt.Println("Job executed after 100 ms")
		close(done)
	}, 100)
	<-done // wait for the job to run before exiting
	fmt.Println("Main: job completed, exiting")
}
