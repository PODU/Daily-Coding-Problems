// Day 361: Mastermind feasibility.
// Brute-force all 6-position codes with distinct digits; keep one that matches
// every guess's score. Time O(P*G*6) with P=10*9*8*7*6*5=151200, Space O(1).
#include <bits/stdc++.h>
using namespace std;

int scoreOf(const string& a, const string& b) {
    int s = 0;
    for (int i = 0; i < 6; i++) if (a[i] == b[i]) s++;
    return s;
}

bool feasible(const vector<pair<string,int>>& guesses) {
    string code(6, '0');
    function<bool(int,int)> rec = [&](int pos, int used) -> bool {
        if (pos == 6) {
            for (auto& g : guesses)
                if (scoreOf(code, g.first) != g.second) return false;
            return true;
        }
        for (int d = 0; d < 10; d++)
            if (!(used & (1 << d))) {
                code[pos] = char('0' + d);
                if (rec(pos + 1, used | (1 << d))) return true;
            }
        return false;
    };
    return rec(0, 0);
}

string pad6(long n) {
    string s = to_string(n);
    return string(6 - s.size(), '0') + s;
}

int main() {
    vector<pair<string,int>> g1 = {{pad6(175286),2},{pad6(293416),3},{pad6(654321),0}};
    vector<pair<string,int>> g2 = {{pad6(123456),4},{pad6(345678),4},{pad6(567890),4}};
    cout << (feasible(g1) ? "True" : "False") << "\n";
    cout << (feasible(g2) ? "True" : "False") << "\n";
    return 0;
}
