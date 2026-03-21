// Word ladder: BFS over words differing by one letter. Time O(N*L*26).
function wordLadder(start, end, dictionary) {
  const dict = new Set(dictionary);
  if (!dict.has(end)) return null;
  const queue = [[start]];
  const seen = new Set([start]);
  while (queue.length) {
    const path = queue.shift();
    const word = path[path.length - 1];
    if (word === end) return path;
    for (let i = 0; i < word.length; i++) {
      for (let c = 97; c <= 122; c++) {
        const nxt = word.slice(0, i) + String.fromCharCode(c) + word.slice(i + 1);
        if (dict.has(nxt) && !seen.has(nxt)) {
          seen.add(nxt);
          queue.push([...path, nxt]);
        }
      }
    }
  }
  return null;
}

console.log(wordLadder("dog", "cat", ["dot", "dop", "dat", "cat"]));
console.log(wordLadder("dog", "cat", ["dot", "tod", "dat", "dar"]));
