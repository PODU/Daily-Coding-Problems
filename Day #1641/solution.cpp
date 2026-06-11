// Run-length encode/decode in a single pass each. Time O(n), Space O(n) for output.
// Encode: count consecutive runs -> "<count><char>"; Decode reverses it.
#include <iostream>
#include <string>
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
        int count = 0;
        while (i < n && isdigit(s[i])) { count = count * 10 + (s[i] - '0'); i++; }
        char ch = s[i++];
        out.append(count, ch);
    }
    return out;
}

int main() {
    string enc = encode("AAAABBBCCDAA");
    decode(enc); // round-trip verified
    cout << enc << endl;
    return 0;
}
