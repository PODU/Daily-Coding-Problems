// Hex string -> bytes -> standard Base64 (with '=' padding).
// Time: O(n), Space: O(n).  Note: canonical Base64 of "deadbeef" is "3q2+7w==".
const B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

function hexToBase64(h) {
  const b = [];
  for (let i = 0; i < h.length; i += 2) b.push(parseInt(h.substr(i, 2), 16));
  let out = "";
  for (let i = 0; i < b.length; i += 3) {
    const rem = b.length - i;
    let n = b[i] << 16;
    if (rem > 1) n |= b[i + 1] << 8;
    if (rem > 2) n |= b[i + 2];
    out += B64[(n >> 18) & 63];
    out += B64[(n >> 12) & 63];
    out += rem > 1 ? B64[(n >> 6) & 63] : "=";
    out += rem > 2 ? B64[n & 63] : "=";
  }
  return out;
}

console.log(hexToBase64("deadbeef"));
