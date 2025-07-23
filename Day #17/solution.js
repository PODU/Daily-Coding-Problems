// Approach: Single pass, track running path length per depth via a map/stack. O(n) time, O(d) space.

function lengthLongestPath(input) {
  const lenAtDepth = { "-1": 0 };
  let maxLen = 0;
  for (const line of input.split("\n")) {
    let depth = 0;
    while (depth < line.length && line[depth] === "\t") depth++;
    const name = line.slice(depth);
    const isFile = name.includes(".");
    const curLen = lenAtDepth[depth - 1] + name.length;
    if (isFile) {
      maxLen = Math.max(maxLen, curLen);
    } else {
      lenAtDepth[depth] = curLen + 1; // +1 for '/'
    }
  }
  return maxLen;
}

const input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
console.log(lengthLongestPath(input));
