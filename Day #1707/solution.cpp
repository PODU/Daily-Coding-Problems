// Elo rating: expected = 1/(1+10^((Rb-Ra)/400)); delta = K*(score-expected), zero-sum. O(1) per game.
#include <bits/stdc++.h>
using namespace std;

struct EloSystem {
    map<string, double> ratings;
    double K = 32.0;
    void add(const string& p, double r = 1200.0) { ratings[p] = r; }
    static double expected(double ra, double rb) {
        return 1.0 / (1.0 + pow(10.0, (rb - ra) / 400.0));
    }
    void recordGame(const string& w, const string& l) {
        double rw = ratings[w], rl = ratings[l];
        double ew = expected(rw, rl);
        double delta = K * (1.0 - ew);
        ratings[w] = rw + delta;
        ratings[l] = rl - delta;
        cout << w << " beats " << l << ": "
             << w << " " << (long)llround(rw) << "->" << (long)llround(rw + delta) << ", "
             << l << " " << (long)llround(rl) << "->" << (long)llround(rl - delta) << "\n";
    }
};

int main() {
    EloSystem e;
    e.add("A"); e.add("B"); e.add("C"); e.add("D");
    e.recordGame("A", "B");
    e.recordGame("A", "C");
    e.recordGame("D", "A");
    cout << "Final ratings:\n";
    for (auto& p : e.ratings)
        cout << p.first << " " << (long)llround(p.second) << "\n";
    return 0;
}
