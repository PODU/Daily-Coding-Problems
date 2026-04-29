// Day 1441: Validate UTF-8 encoding from an array of byte values.
// Approach: scan bytes, read leading-one count of lead byte, verify
// continuation bytes start with 10. Time O(n), Space O(1).
function validUtf8(data) {
  let i = 0;
  const n = data.length;
  while (i < n) {
    const b = data[i] & 0xff;
    let cnt;
    if (b >> 7 === 0b0) cnt = 1;
    else if (b >> 5 === 0b110) cnt = 2;
    else if (b >> 4 === 0b1110) cnt = 3;
    else if (b >> 3 === 0b11110) cnt = 4;
    else return false;
    if (i + cnt > n) return false;
    for (let j = 1; j < cnt; j++) {
      if ((data[i + j] & 0xff) >> 6 !== 0b10) return false;
    }
    i += cnt;
  }
  return true;
}

const euro = [226, 130, 172]; // 11100010 10000010 10101100
console.log(validUtf8(euro) ? "True" : "False");
