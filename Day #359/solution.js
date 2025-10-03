// Recover digits from scrambled letters: use unique marker letters (z,w,u,x,g) then derive the rest. O(N) time, O(1) space.
function recover(s) {
  const c = {};
  for (const ch of s) c[ch] = (c[ch] || 0) + 1;
  const g = ch => c[ch] || 0;
  const d = new Array(10).fill(0);
  d[0] = g('z');
  d[2] = g('w');
  d[4] = g('u');
  d[6] = g('x');
  d[8] = g('g');
  d[3] = g('h') - d[8];
  d[5] = g('f') - d[4];
  d[7] = g('s') - d[6];
  d[1] = g('o') - d[0] - d[2] - d[4];
  d[9] = g('i') - d[5] - d[6] - d[8];
  let res = '';
  for (let i = 0; i < 10; i++) res += String(i).repeat(d[i]);
  return res;
}
console.log(recover("niesevehrtfeev"));
