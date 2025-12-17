// Day 759: Generate all valid IPv4 addresses from a digit string (backtracking).
// At most 3^3 splits; Time: O(1) practically (bounded), Space: O(#results).
#include <bits/stdc++.h>
using namespace std;

bool validOctet(const string& s) {
    if (s.empty() || s.size() > 3) return false;
    if (s.size() > 1 && s[0] == '0') return false;   // no leading zero
    return stoi(s) <= 255;
}

void backtrack(const string& s, int start, int part, vector<string>& cur,
               vector<string>& res) {
    if (part == 4) {
        if (start == (int)s.size()) {
            res.push_back(cur[0] + "." + cur[1] + "." + cur[2] + "." + cur[3]);
        }
        return;
    }
    for (int len = 1; len <= 3 && start + len <= (int)s.size(); ++len) {
        string seg = s.substr(start, len);
        if (validOctet(seg)) {
            cur.push_back(seg);
            backtrack(s, start + len, part + 1, cur, res);
            cur.pop_back();
        }
    }
}

vector<string> restoreIps(const string& s) {
    vector<string> res, cur;
    backtrack(s, 0, 0, cur, res);
    return res;
}

int main() {
    vector<string> res = restoreIps("2542540123");
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << "'" << res[i] << "'" << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";   // ['254.25.40.123', '254.254.0.123']
    return 0;
}
