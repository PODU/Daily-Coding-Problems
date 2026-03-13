// Reverse all 32 bits by shifting LSB of input into LSB-first of result (>>> 0 for unsigned).
// Time: O(1) (32 steps); Space: O(1).
function reverseBits(x) {
    let r = 0;
    for (let i = 0; i < 32; i++) {
        r = (r << 1) | (x & 1);
        x >>>= 1;
    }
    return r >>> 0;
}

function toGrouped(x) {
    const bits = (x >>> 0).toString(2).padStart(32, "0");
    return bits.match(/.{4}/g).join(" ");
}

const input = 0xF0F0F0F0;
console.log("Input: ", toGrouped(input));
console.log(toGrouped(reverseBits(input)));
