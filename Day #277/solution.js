// Day 277: Validate UTF-8 from byte-value array. Single pass.
// Time O(N), Space O(1). Only low 8 bits of each integer are used.
function validUTF8(data) {
  let remaining = 0;
  for (const v of data) {
    const b = v & 0xff;
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

console.log(validUTF8([0b11100010, 0b10000010, 0b10101100])); // true (Euro sign)
console.log(validUTF8([0b11101011, 0b10001100, 0b00000100])); // false
