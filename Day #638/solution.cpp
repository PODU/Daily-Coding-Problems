// Day 638: Reverse the words in a string (in-place on a char buffer).
// Approach: reverse whole string, then reverse each word back.
// Time: O(n), Space: O(1) extra.
#include <bits/stdc++.h>
using namespace std;

void reverseWords(string& s) {
    reverse(s.begin(), s.end());
    int n = s.size(), i = 0;
    while (i < n) {
        int j = i;
        while (j < n && s[j] != ' ') j++;
        reverse(s.begin() + i, s.begin() + j);
        i = j + 1;
    }
}

int main() {
    string s = "hello world here";
    reverseWords(s);
    cout << s << "\n"; // here world hello
    return 0;
}
