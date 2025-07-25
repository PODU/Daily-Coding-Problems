// Run-length encoding/decoding in a single pass.
// Time: O(n), Space: O(n) for output.
#include <iostream>
#include <string>
#include <cctype>
using namespace std;

string encode(const string& s) {
    string res;
    int n = s.size();
    for (int i = 0; i < n;) {
        int j = i;
        while (j < n && s[j] == s[i]) j++;
        res += to_string(j - i) + s[i];
        i = j;
    }
    return res;
}

string decode(const string& s) {
    string res;
    int n = s.size();
    for (int i = 0; i < n;) {
        int count = 0;
        while (i < n && isdigit((unsigned char)s[i])) {
            count = count * 10 + (s[i] - '0');
            i++;
        }
        char c = s[i++];
        res += string(count, c);
    }
    return res;
}

int main() {
    string input = "AAAABBBCCDAA";
    string enc = encode(input);
    cout << enc << "\n";
    cout << decode(enc) << "\n";
    return 0;
}
