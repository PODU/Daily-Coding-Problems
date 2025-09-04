// Day 213: Generate all valid IP addresses from a digit string.
// Approach: backtracking over the 3 dot positions; each segment is 1-3 digits, 0-255, no leading zeros.
// Time O(1) effectively (string length <= 12, bounded branching), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool valid(const string& seg) {
    if (seg.empty() || seg.size() > 3) return false;
    if (seg.size() > 1 && seg[0] == '0') return false;
    return stoi(seg) <= 255;
}

void solve(const string& s, int start, int part, vector<string>& cur, vector<string>& res) {
    if (part == 4) {
        if (start == (int)s.size()) {
            res.push_back(cur[0] + "." + cur[1] + "." + cur[2] + "." + cur[3]);
        }
        return;
    }
    for (int len = 1; len <= 3 && start + len <= (int)s.size(); len++) {
        string seg = s.substr(start, len);
        if (!valid(seg)) continue;
        cur.push_back(seg);
        solve(s, start + len, part + 1, cur, res);
        cur.pop_back();
    }
}

vector<string> restore(const string& s) {
    vector<string> res, cur;
    solve(s, 0, 0, cur, res);
    return res;
}

int main() {
    auto r = restore("2542540123");
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << "'" << r[i] << "'" << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
