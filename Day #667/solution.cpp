// Day 667: Simplified Elo rating. Expected score E = 1/(1+10^((Rb-Ra)/400)),
// update R += K*(actual - expected). Underdog gains more. Each update O(1).
#include <bits/stdc++.h>
using namespace std;

struct Elo {
    double K = 32, start = 1200;
    unordered_map<string, double> r;
    double rating(const string& p) { if (!r.count(p)) r[p] = start; return r[p]; }
    void game(const string& winner, const string& loser) {
        double ra = rating(winner), rb = rating(loser);
        double ea = 1.0 / (1.0 + pow(10.0, (rb - ra) / 400.0));
        double eb = 1.0 - ea;
        r[winner] = ra + K * (1 - ea);
        r[loser]  = rb + K * (0 - eb);
    }
};

int main() {
    Elo e;
    e.r["A"] = 1200; e.r["B"] = 2000;
    e.game("A", "B"); // underdog A beats B
    cout << fixed << setprecision(1);
    cout << "A: " << e.rating("A") << "\n"; // ~1231.5
    cout << "B: " << e.rating("B") << "\n"; // ~1968.5
    return 0;
}
