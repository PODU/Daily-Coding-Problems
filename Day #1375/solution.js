// Word circle: backtracking to order all words so each last char == next first char,
// and the last wraps to the first. Time O(n!) worst, Space O(n). (n small)
function circle(words) {
  const n = words.length;
  const used = new Array(n).fill(false);
  const order = [0];
  used[0] = true;

  const last = (w) => w[w.length - 1];

  const bt = () => {
    if (order.length === n) return last(words[order[order.length - 1]]) === words[order[0]][0];
    const need = last(words[order[order.length - 1]]);
    for (let i = 0; i < n; i++) {
      if (!used[i] && words[i][0] === need) {
        used[i] = true; order.push(i);
        if (bt()) return true;
        order.pop(); used[i] = false;
      }
    }
    return false;
  };

  return bt() ? order : null;
}

const words = ["chair", "height", "racket", "touch", "tunic"];
const order = circle(words);
if (order === null) console.log("Cannot form a circle");
else console.log(order.map((i) => words[i]).join(" --> ") + " --> " + words[order[0]]);
