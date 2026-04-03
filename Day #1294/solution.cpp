// Day 1294: Run-length encoding and decoding for alphabetic strings.
// Single linear scan for each direction. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

string encode(const string& s) {
    string out;
    int n = s.size();
    for (int i = 0; i < n;) {
        int j = i;
        while (j < n && s[j] == s[i]) j++;
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
    string in = "AAAABBBCCDAA";
    string e = encode(in);
    cout << e << endl;          // 4A3B2C1D2A
    cout << decode(e) << endl;  // AAAABBBCCDAA
    return 0;
}
