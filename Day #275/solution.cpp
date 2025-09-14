// Day 275: Nth term of look-and-say (term 1 = "1").
// Build each term by run-length encoding the previous. Time O(N * len), Space O(len).
#include <bits/stdc++.h>
using namespace std;

string lookAndSay(int n) {
    string cur = "1";
    for (int t = 1; t < n; t++) {
        string nxt;
        int i = 0, m = (int)cur.size();
        while (i < m) {
            int j = i;
            while (j < m && cur[j] == cur[i]) j++;
            nxt += to_string(j - i);
            nxt += cur[i];
            i = j;
        }
        cur = nxt;
    }
    return cur;
}

int main() {
    cout << lookAndSay(5) << "\n"; // 111221
    return 0;
}
