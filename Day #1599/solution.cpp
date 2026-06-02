// Generate all valid IPv4 addresses by backtracking: place 3 dots, each
// segment len 1..3, value 0..255, no leading zeros. Time O(1) (bounded).
#include <bits/stdc++.h>
using namespace std;

void backtrack(const string& s, int start, int part, vector<string>& cur, vector<string>& res) {
    if (part == 4) {
        if (start == (int)s.size()) {
            res.push_back(cur[0] + "." + cur[1] + "." + cur[2] + "." + cur[3]);
        }
        return;
    }
    for (int len = 1; len <= 3 && start + len <= (int)s.size(); len++) {
        string seg = s.substr(start, len);
        if (seg.size() > 1 && seg[0] == '0') break;       // leading zero
        if (stoi(seg) > 255) break;                        // out of range
        cur.push_back(seg);
        backtrack(s, start + len, part + 1, cur, res);
        cur.pop_back();
    }
}

vector<string> restoreIp(const string& s) {
    vector<string> res, cur;
    backtrack(s, 0, 0, cur, res);
    return res;
}

int main() {
    string s = "2542540123";
    vector<string> res = restoreIp(s);

    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        if (i) cout << ", ";
        cout << "'" << res[i] << "'";
    }
    cout << "]\n";
    return 0;
}
