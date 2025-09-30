// Largest rectangle in histogram via monotonic increasing stack. Time O(N), Space O(N).

function largestRectangle(heights) {
  const stack = []; // indices of increasing bars
  let best = 0;
  for (let i = 0; i <= heights.length; i++) {
    const h = i < heights.length ? heights[i] : 0;
    while (stack.length && heights[stack[stack.length - 1]] >= h) {
      const height = heights[stack.pop()];
      const width = stack.length ? i - stack[stack.length - 1] - 1 : i;
      best = Math.max(best, height * width);
    }
    stack.push(i);
  }
  return best;
}

function main() {
  console.log(largestRectangle([1, 3, 2, 5]));
}

main();
