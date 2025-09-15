// Day 277: Validate UTF-8 from byte-value array. Single pass.
// Time O(N), Space O(1). Only low 8 bits of each integer are used.
#include <bits/stdc++.h>
using namespace std;

bool validUTF8(const vector<int>& data) {
    int remaining = 0; // continuation bytes still expected
    for (int v : data) {
        int b = v & 0xFF;
        if (remaining == 0) {
            if ((b >> 7) == 0) remaining = 0;            // 0xxxxxxx
            else if ((b >> 5) == 0b110) remaining = 1;   // 110xxxxx
            else if ((b >> 4) == 0b1110) remaining = 2;  // 1110xxxx
            else if ((b >> 3) == 0b11110) remaining = 3; // 11110xxx
            else return false;
        } else {
            if ((b >> 6) != 0b10) return false;          // 10xxxxxx
            remaining--;
        }
    }
    return remaining == 0;
}

int main() {
    cout << boolalpha;
    cout << validUTF8({0b11100010, 0b10000010, 0b10101100}) << "\n"; // true (Euro sign)
    cout << validUTF8({0b11101011, 0b10001100, 0b00000100}) << "\n"; // false
    return 0;
}
