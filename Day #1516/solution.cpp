// Reverse words in a space-delimited string.
// Approach: reverse whole char array, then reverse each word in place (in-place on mutable string).
// Time: O(n), Space: O(1) extra.
#include <bits/stdc++.h>
using namespace std;

void reverseRange(string &s, int i, int j) {
    while (i < j) swap(s[i++], s[j--]);
}

string reverseWords(string s) {
    int n = s.size();
    reverseRange(s, 0, n - 1);
    int start = 0;
    for (int i = 0; i <= n; i++) {
        if (i == n || s[i] == ' ') {
            reverseRange(s, start, i - 1);
            start = i + 1;
        }
    }
    return s;
}

int main() {
    string in = "hello world here";
    cout << '"' << reverseWords(in) << '"' << endl; // "here world hello"
    return 0;
}
