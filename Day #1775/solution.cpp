// Day 1775: Mastermind consistency. Brute-force every 6-digit code with distinct
// digits (P(10,6)=151200); a code is valid if it reproduces every guess's score.
// O(P(10,6) * G) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

struct Guess { int d[6]; int score; };

int code[6];
bool used[10];

bool checkAll(const vector<Guess>& gs) {
    for (const auto& g : gs) {
        int m = 0;
        for (int i = 0; i < 6; ++i) if (code[i] == g.d[i]) ++m;
        if (m != g.score) return false;
    }
    return true;
}

bool consistent(const vector<Guess>& gs);

bool rec(int pos, const vector<Guess>& gs) {
    if (pos == 6) return checkAll(gs);
    for (int dgt = 0; dgt < 10; ++dgt) {
        if (used[dgt]) continue;
        used[dgt] = true; code[pos] = dgt;
        if (rec(pos + 1, gs)) { used[dgt] = false; return true; }
        used[dgt] = false;
    }
    return false;
}

bool consistent(const vector<Guess>& gs) {
    memset(used, 0, sizeof(used));
    return rec(0, gs);
}

Guess mk(long long num, int score) {
    Guess g; g.score = score;
    for (int i = 5; i >= 0; --i) { g.d[i] = num % 10; num /= 10; }
    return g;
}

int main() {
    vector<Guess> a = {mk(175286,2), mk(293416,3), mk(654321,0)};
    vector<Guess> b = {mk(123456,4), mk(345678,4), mk(567890,4)};
    cout << (consistent(a) ? "True" : "False") << "\n";
    cout << (consistent(b) ? "True" : "False") << "\n";
    return 0;
}
