// Run-length encoding/decoding. Single pass over the string.
// Time: O(n) encode/decode, Space: O(n) for output.
#include <bits/stdc++.h>
using namespace std;

string encode(const string& s) {
    string out;
    int n = s.size();
    for (int i = 0; i < n;) {
        int j = i;
        while (j < n && s[j] == s[i]) j++;
        out += to_string(j - i);
        out += s[i];
        i = j;
    }
    return out;
}

string decode(const string& s) {
    string out;
    int n = s.size();
    for (int i = 0; i < n;) {
        int cnt = 0;
        while (i < n && isdigit((unsigned char)s[i])) { cnt = cnt * 10 + (s[i] - '0'); i++; }
        char c = s[i++];
        out.append(cnt, c);
    }
    return out;
}

int main() {
    string input = "AAAABBBCCDAA";
    string enc = encode(input);
    cout << enc << "\n";
    cout << decode(enc) << "\n";
    return 0;
}
