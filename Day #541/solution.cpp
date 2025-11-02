// Day 541: Run-length encoding/decoding. Scan runs to build count+char; parse digits to expand.
// Time O(n) encode, O(output) decode. Space O(n).
#include <bits/stdc++.h>
using namespace std;

string encode(const string& s) {
    string out;
    for (size_t i = 0; i < s.size();) {
        size_t j = i;
        while (j < s.size() && s[j] == s[i]) j++;
        out += to_string(j - i) + s[i];
        i = j;
    }
    return out;
}

string decode(const string& s) {
    string out;
    int count = 0;
    for (char c : s) {
        if (isdigit((unsigned char)c)) count = count * 10 + (c - '0');
        else { out.append(count, c); count = 0; }
    }
    return out;
}

int main() {
    string original = "AAAABBBCCDAA";
    string enc = encode(original);
    cout << enc << "\n";                      // 4A3B2C1D2A
    cout << (decode(enc) == original ? "true" : "false") << "\n";
    return 0;
}
