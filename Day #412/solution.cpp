// Day 412: Nth term of the look-and-say sequence via run-length encoding.
// Build each term from the previous by counting consecutive runs. O(N * L) time where L = term length, O(L) space.
#include <bits/stdc++.h>
using namespace std;

string lookAndSay(int n) {
    string cur = "1";
    for (int t = 1; t < n; t++) {
        string next;
        int i = 0, m = cur.size();
        while (i < m) {
            int j = i;
            while (j < m && cur[j] == cur[i]) j++;
            next += to_string(j - i);
            next += cur[i];
            i = j;
        }
        cur = next;
    }
    return cur;
}

int main() {
    int n = 4;
    cout << lookAndSay(n) << endl;
    return 0;
}
