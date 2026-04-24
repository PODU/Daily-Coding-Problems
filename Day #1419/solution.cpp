// Day 1419: reverse the order of space-delimited words, in-place.
// Approach: reverse whole char array, then reverse each word. O(n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

void reverseRange(string& s, int i, int j) {
    while (i < j) swap(s[i++], s[j--]);
}

void reverseWords(string& s) {
    reverseRange(s, 0, (int)s.size() - 1);
    int start = 0, n = (int)s.size();
    for (int i = 0; i <= n; ++i) {
        if (i == n || s[i] == ' ') {
            reverseRange(s, start, i - 1);
            start = i + 1;
        }
    }
}

int main() {
    string s = "hello world here";
    reverseWords(s);
    cout << s << "\n"; // here world hello
    return 0;
}
