// Generate valid IP addresses via backtracking over 4 octets (each 0-255, no leading zeros).
// Time O(1) (bounded by string length <= 12), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

bool valid(const string& seg) {
    if (seg.empty() || seg.size() > 3) return false;
    if (seg.size() > 1 && seg[0] == '0') return false;
    return stoi(seg) <= 255;
}

void backtrack(const string& s, int start, int part, vector<string>& cur, vector<string>& res) {
    if (part == 4) {
        if (start == (int)s.size()) {
            string ip = cur[0] + "." + cur[1] + "." + cur[2] + "." + cur[3];
            res.push_back(ip);
        }
        return;
    }
    for (int len = 1; len <= 3 && start + len <= (int)s.size(); len++) {
        string seg = s.substr(start, len);
        if (valid(seg)) {
            cur.push_back(seg);
            backtrack(s, start + len, part + 1, cur, res);
            cur.pop_back();
        }
    }
}

int main() {
    string s = "2542540123";
    vector<string> cur, res;
    backtrack(s, 0, 0, cur, res);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << "'" << res[i] << "'" << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
}
