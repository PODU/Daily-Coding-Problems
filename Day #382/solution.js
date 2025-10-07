// Base64 -> bytes -> hex. Bit-accumulator decode (tolerates padding/whitespace).
// Time: O(n), Space: O(n).
const B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const VAL = {};
for (let i = 0; i < B64.length; i++) VAL[B64[i]] = i;

function base64ToHex(s) {
  let bits = 0, nbits = 0, out = "";
  for (const c of s) {
    const v = VAL[c];
    if (v === undefined) continue;
    bits = (bits << 6) | v; nbits += 6;
    if (nbits >= 8) {
      nbits -= 8;
      const b = (bits >> nbits) & 0xff;
      out += b.toString(16).padStart(2, "0");
    }
  }
  return out;
}

console.log(base64ToHex("3q2+7w="));
