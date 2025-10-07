// Brute-force all 256 single-byte keys; score by letters/spaces to pick plaintext.
// Time: O(256 * n), Space: O(n).
function hexToBytes(h) {
  const b = [];
  for (let i = 0; i < h.length; i += 2) b.push(parseInt(h.substr(i, 2), 16));
  return b;
}

function score(codes) {
  for (const c of codes) if (c < 32 || c > 126) return -1;
  let sc = 0;
  for (const c of codes) if ((c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c === 32) sc++;
  return sc;
}

function decrypt(hex) {
  const bytes = hexToBytes(hex);
  let best = "", bestScore = -1;
  for (let k = 0; k < 256; k++) {
    const codes = bytes.map(b => b ^ k);
    const sc = score(codes);
    if (sc > bestScore) {
      bestScore = sc;
      best = String.fromCharCode(...codes);
    }
  }
  return best;
}

const h = "7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f";
console.log(decrypt(h));
