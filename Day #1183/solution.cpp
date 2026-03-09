// Day 1183: Generate all N-digit strobogrammatic numbers (same when rotated 180).
// Recursively build from outside in using rotation pairs; drop leading zeros.
// Time O(output size), Space O(N) recursion depth.
#include <bits/stdc++.h>
using namespace std;

vector<string> helper(int m) {
    if (m == 0) return {""};
    if (m == 1) return {"0", "1", "8"};
    vector<pair<char,char>> pairs = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};
    vector<string> inner = helper(m - 2), res;
    for (auto& s : inner)
        for (auto& p : pairs)
            res.push_back(string(1, p.first) + s + string(1, p.second));
    return res;
}

vector<long long> strobogrammatic(int n) {
    vector<long long> out;
    for (auto& s : helper(n))
        if (!(s.size() > 1 && s[0] == '0') && s != "0")
            out.push_back(stoll(s));
    sort(out.begin(), out.end());
    return out;
}

bool isStrobo(const string& s) {
    map<char,char> rot = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};
    for (int l = 0, r = (int)s.size() - 1; l <= r; l++, r--) {
        if (!rot.count(s[l]) || rot[s[l]] != s[r]) return false;
    }
    return true;
}

int main() {
    auto v = strobogrammatic(2);
    cout << "2-digit strobogrammatic numbers: [";
    for (size_t i = 0; i < v.size(); i++) cout << v[i] << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
    cout << "16891 is strobogrammatic: " << (isStrobo("16891") ? "true" : "false") << "\n";
    return 0;
}
