// Look-and-say: build each term by run-length encoding the previous. Time O(total digits), space O(len).
#include <bits/stdc++.h>
using namespace std;

string lookAndSay(int n) {
    string cur = "1";
    for (int k = 1; k < n; k++) {
        string next;
        int i = 0, len = cur.size();
        while (i < len) {
            int j = i;
            while (j < len && cur[j] == cur[i]) j++;
            next += to_string(j - i);
            next += cur[i];
            i = j;
        }
        cur = next;
    }
    return cur;
}

int main() {
    cout << lookAndSay(4) << endl; // 1, 11, 21, 1211
    return 0;
}
