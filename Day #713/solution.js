// Day 713: Normalize absolute Unix path. Split on '/', use a stack: skip ""/".",
// pop on "..". Preserve a trailing slash if the input had one. Time O(n).
function normalize(path) {
  const stk = [];
  for (const comp of path.split('/')) {
    if (comp === '' || comp === '.') continue;
    if (comp === '..') { if (stk.length) stk.pop(); }
    else stk.push(comp);
  }
  let res = '/' + stk.join('/');
  const trailing = path.length > 1 && path[path.length - 1] === '/';
  if (trailing && res !== '/' && res[res.length - 1] !== '/') res += '/';
  return res;
}

console.log('"' + normalize("/usr/bin/../bin/./scripts/../") + '"');
