// Day 961: Normalize an absolute Unix path resolving "." and "..".
// Approach: split by '/', use a stack. Time O(n), Space O(n).

function simplifyPath(path) {
  const stack = [];
  for (const seg of path.split('/')) {
    if (seg === '' || seg === '.') continue;
    if (seg === '..') { if (stack.length) stack.pop(); }
    else stack.push(seg);
  }
  let res = '/' + stack.join('/');
  if (path.length && path[path.length - 1] === '/' && res !== '/') res += '/';
  return res;
}

console.log('"' + simplifyPath('/usr/bin/../bin/./scripts/../') + '"');
