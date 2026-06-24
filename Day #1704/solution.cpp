// Lazy bartender = min set cover (NP-hard). Greedy: repeatedly pick drink covering
// most uncovered customers. Time O(D*C) per pick -> O(D^2*C), Space O(D*C).
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<int, vector<int>> preferences = {
        {0, {0,1,3,6}}, {1, {1,4,7}}, {2, {2,4,7,5}}, {3, {3,2,5}}, {4, {5,8}}
    };
    // Build drink -> set of customers
    map<int, set<int>> drinkToCust;
    set<int> uncovered;
    for (auto& [cust, drinks] : preferences) {
        uncovered.insert(cust);
        for (int d : drinks) drinkToCust[d].insert(cust);
    }
    int learned = 0;
    while (!uncovered.empty()) {
        int bestDrink = -1, bestCount = 0;
        for (auto& [drink, custs] : drinkToCust) {
            int cnt = 0;
            for (int c : custs) if (uncovered.count(c)) cnt++;
            if (cnt > bestCount) { bestCount = cnt; bestDrink = drink; }
        }
        if (bestDrink == -1) break; // some customer has no drinks
        for (int c : drinkToCust[bestDrink]) uncovered.erase(c);
        learned++;
    }
    cout << learned << "\n";
    return 0;
}
