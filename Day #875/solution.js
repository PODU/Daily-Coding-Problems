// Longest absolute file path: track path length per depth in one pass.
// Time O(n), Space O(d) for depth d.

function lengthLongestPath(input) {
  const pathLen = { 0: 0 };
  let maxLen = 0;
  for (const line of input.split("\n")) {
    let depth = 0;
    while (depth < line.length && line[depth] === "\t") depth++;
    const name = line.slice(depth);
    if (name.indexOf(".") >= 0) {
      maxLen = Math.max(maxLen, pathLen[depth] + name.length);
    } else {
      pathLen[depth + 1] = pathLen[depth] + name.length + 1; // +1 for '/'
    }
  }
  return maxLen;
}

const input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
console.log(lengthLongestPath(input));
