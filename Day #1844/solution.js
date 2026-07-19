// Day 1844: Largest rectangle in a histogram via a monotonic increasing stack.
// Time O(N), Space O(N).

function largestRectangle(heights) {
  const h = [...heights, 0]; // sentinel
  const stack = [];
  let best = 0;
  for (let i = 0; i < h.length; i++) {
    while (stack.length && h[stack[stack.length - 1]] >= h[i]) {
      const height = h[stack.pop()];
      const left = stack.length ? stack[stack.length - 1] : -1;
      const width = i - left - 1;
      best = Math.max(best, height * width);
    }
    stack.push(i);
  }
  return best;
}

function main() {
  console.log(largestRectangle([1, 3, 2, 5])); // 6
}

main();
