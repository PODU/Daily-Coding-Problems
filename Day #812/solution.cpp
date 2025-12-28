// Simplified Elo rating system. Expected score logistic, K=32 update on win/loss.
// record_game adjusts both players. Time O(1) per game, Space O(players).
#include <bits/stdc++.h>
using namespace std;

struct Elo {
    map<string, double> ratings;
    static constexpr double K = 32.0;
    void add(const string& name, double r = 1200.0) { ratings[name] = r; }
    double expected(double ra, double rb) { return 1.0 / (1.0 + pow(10.0, (rb - ra) / 400.0)); }
    void record_game(const string& winner, const string& loser) {
        double ra = ratings[winner], rb = ratings[loser];
        double ea = expected(ra, rb), eb = expected(rb, ra);
        ratings[winner] = ra + K * (1.0 - ea);
        ratings[loser]  = rb + K * (0.0 - eb);
    }
};

int main() {
    cout << fixed << setprecision(2);
    Elo e;
    e.add("A"); e.add("B");
    cout << "Initial: A=" << e.ratings["A"] << " B=" << e.ratings["B"] << "\n";
    e.record_game("B", "A"); // B beats A, equal ratings
    cout << "After B beats A (equal): A=" << e.ratings["A"] << " B=" << e.ratings["B"] << "\n";

    // Underdog upset: C low, D high; C beats D -> larger gain.
    Elo e2;
    e2.add("C", 1000.0); e2.add("D", 1600.0);
    cout << "Initial: C=" << e2.ratings["C"] << " D=" << e2.ratings["D"] << "\n";
    e2.record_game("C", "D");
    cout << "After underdog C beats D: C=" << e2.ratings["C"] << " D=" << e2.ratings["D"] << "\n";
    cout << "Underdog gained " << (e2.ratings["C"] - 1000.0)
         << " vs even-match gain " << (e.ratings["B"] - 1200.0) << "\n";
    return 0;
}
