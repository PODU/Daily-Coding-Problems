// UTF-8 validation: read lead byte, count leading ones (1->1byte, 2..4 multi), verify continuation bytes start with 10.
// Time O(n), Space O(1).
function validUtf8(data) {
  let i = 0;
  const n = data.length;
  while (i < n) {
    const b = data[i] & 0xff;
    let cnt;
    if ((b & 0x80) === 0x00) cnt = 1;
    else if ((b & 0xe0) === 0xc0) cnt = 2;
    else if ((b & 0xf0) === 0xe0) cnt = 3;
    else if ((b & 0xf8) === 0xf0) cnt = 4;
    else return false;
    if (i + cnt > n) return false;
    for (let k = 1; k < cnt; k++) {
      if ((data[i + k] & 0xc0) !== 0x80) return false;
    }
    i += cnt;
  }
  return true;
}

console.log(validUtf8([226, 130, 172]));                       // true
console.log(validUtf8([0b11110101, 0b10000010, 0b00000010]));  // false
