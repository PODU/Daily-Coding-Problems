// Restore IP addresses via backtracking over 4 octets. Time O(1) (<=3^3 splits),
// Space O(1) recursion depth. Each octet in [0,255], no leading zeros (except "0").
#include <bits/stdc++.h>
using namespace std;

bool valid(const string& s) {
    if (s.empty() || s.size() > 3) return false;
    if (s.size() > 1 && s[0] == '0') return false;
    return stoi(s) <= 255;
}

void bt(const string& s, int start, int part, vector<string>& cur, vector<string>& res) {
    if (part == 4) {
        if (start == (int)s.size())
            res.push_back(cur[0] + "." + cur[1] + "." + cur[2] + "." + cur[3]);
        return;
    }
    for (int len = 1; len <= 3 && start + len <= (int)s.size(); len++) {
        string seg = s.substr(start, len);
        if (!valid(seg)) continue;
        cur.push_back(seg);
        bt(s, start + len, part + 1, cur, res);
        cur.pop_back();
    }
}

vector<string> restore(const string& s) {
    vector<string> res, cur;
    bt(s, 0, 0, cur, res);
    return res;
}

int main() {
    auto res = restore("2542540123");
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "'" << res[i] << "'";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
