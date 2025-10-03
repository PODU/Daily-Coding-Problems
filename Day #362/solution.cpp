// Day 362: Strobogrammatic numbers of N digits.
// Recursively build from outside in using rotatable digit pairs; skip leading 0.
// Time O(output size), Space O(N) recursion depth.
#include <bits/stdc++.h>
using namespace std;

vector<string> build(int n, int total) {
    if (n == 0) return {""};
    if (n == 1) return {"0", "1", "8"};
    vector<string> inner = build(n - 2, total);
    vector<pair<char,char>> pairs = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};
    vector<string> res;
    for (auto& s : inner)
        for (auto& p : pairs) {
            if (n == total && p.first == '0') continue; // no leading zero
            res.push_back(string(1, p.first) + s + p.second);
        }
    return res;
}

vector<string> strobogrammatic(int n) { return build(n, n); }

int main() {
    int n = 2;
    auto res = strobogrammatic(n);
    cout << "N=" << n << ": ";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? " " : "\n");
    return 0;
}
