// Longest absolute file path. Parse tab-indented FS, track cumulative path
// length per depth. Time O(n), Space O(depth).
'use strict';

function lengthLongestPath(input) {
  const pathLen = { 0: 0 };
  let best = 0;
  for (const line of input.split('\n')) {
    let level = 0;
    while (level < line.length && line[level] === '\t') level++;
    const name = line.slice(level);
    if (name.includes('.')) {
      best = Math.max(best, pathLen[level] + name.length);
    } else {
      pathLen[level + 1] = pathLen[level] + name.length + 1;
    }
  }
  return best;
}

const s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
console.log(lengthLongestPath(s)); // 32
