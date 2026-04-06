// Look-and-say: build each term by run-length encoding the previous one.
// Time O(sum of term lengths), Space O(length of Nth term).
#include <bits/stdc++.h>
using namespace std;

string lookAndSay(int n) {
    string s = "1";
    for (int i = 1; i < n; i++) {
        string next;
        for (size_t j = 0; j < s.size();) {
            size_t k = j;
            while (k < s.size() && s[k] == s[j]) k++;
            next += to_string(k - j) + s[j];
            j = k;
        }
        s = next;
    }
    return s;
}

int main() {
    cout << lookAndSay(5) << "\n"; // 111221
    return 0;
}
