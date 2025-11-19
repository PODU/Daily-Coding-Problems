// Job scheduler: run f after n ms using setTimeout.
// Schedule is O(1); the Node event loop stays alive until the timer fires.
'use strict';

function schedule(f, n) {
  return setTimeout(f, n);
}

schedule(() => {
  console.log('Job executed after 100 ms');
  // logged after the job runs, before the event loop drains
  console.log('Main: job completed, exiting');
}, 100);
