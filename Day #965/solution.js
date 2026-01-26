// Day 965: Validate whether byte values form a valid UTF-8 encoding.
// Approach: count leading 1s of lead byte, verify continuation bytes. Time O(n), Space O(1).

function validUtf8(data) {
  let remaining = 0;
  for (let b of data) {
    b &= 0xff;
    if (remaining === 0) {
      if (b >> 7 === 0) remaining = 0;
      else if (b >> 5 === 0b110) remaining = 1;
      else if (b >> 4 === 0b1110) remaining = 2;
      else if (b >> 3 === 0b11110) remaining = 3;
      else return false;
    } else {
      if (b >> 6 !== 0b10) return false;
      remaining--;
    }
  }
  return remaining === 0;
}

console.log(validUtf8([0b11100010, 0b10000010, 0b10101100])); // true (Euro sign)
console.log(validUtf8([0b11110000, 0b10000000]));             // false (truncated)
