// Day 803: Does a secret (6 distinct digits) exist matching all guess scores?
// Brute force all 6-digit distinct-digit codes, verify every guess's score.
// Time O(10^6 * G), Space O(G).
#include <bits/stdc++.h>
using namespace std;

array<int, 6> digits(int x) {
    array<int, 6> d{};
    for (int i = 5; i >= 0; i--) { d[i] = x % 10; x /= 10; }
    return d;
}

bool distinct(const array<int, 6>& d) {
    int seen = 0;
    for (int v : d) { if (seen & (1 << v)) return false; seen |= 1 << v; }
    return true;
}

int score(const array<int, 6>& code, const array<int, 6>& guess) {
    int s = 0;
    for (int i = 0; i < 6; i++) if (code[i] == guess[i]) s++;
    return s;
}

bool feasible(const vector<pair<int, int>>& guesses) {
    vector<pair<array<int, 6>, int>> gs;
    for (auto& [g, sc] : guesses) gs.push_back({digits(g), sc});
    for (int code = 0; code <= 999999; code++) {
        auto d = digits(code);
        if (!distinct(d)) continue;
        bool ok = true;
        for (auto& [gd, sc] : gs) if (score(d, gd) != sc) { ok = false; break; }
        if (ok) return true;
    }
    return false;
}

int main() {
    cout << boolalpha;
    cout << feasible({{175286, 2}, {293416, 3}, {654321, 0}}) << "\n"; // true
    cout << feasible({{123456, 4}, {345678, 4}, {567890, 4}}) << "\n"; // false
    return 0;
}
