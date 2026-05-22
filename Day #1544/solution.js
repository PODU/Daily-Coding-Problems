// Longest absolute path to a file in a tab-indented filesystem string.
// Track cumulative path length per depth; a name with '.' is a file.
// Time O(n), Space O(depth).
function longestPath(s) {
  const lens = { 0: 0 };
  let maxLen = 0;
  for (const line of s.split("\n")) {
    let depth = 0;
    while (depth < line.length && line[depth] === "\t") depth++;
    const name = line.slice(depth);
    if (name.includes(".")) {
      maxLen = Math.max(maxLen, lens[depth] + name.length);
    } else {
      lens[depth + 1] = lens[depth] + name.length + 1; // +1 for '/'
    }
  }
  return maxLen;
}

const s = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
console.log(longestPath(s));
