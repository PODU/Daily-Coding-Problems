// Streaming vote tally: hash set of seen voters (duplicate -> fraud, vote dropped),
// hash map candidate->count, top 3 computed on demand. Time O(n + k log k), Space O(n+k).
#include <iostream>
#include <unordered_set>
#include <unordered_map>
#include <string>
#include <vector>
#include <algorithm>

int main() {
    std::vector<std::pair<std::string, std::string>> stream = {
        {"v1", "A"}, {"v2", "A"}, {"v3", "B"}, {"v4", "C"},
        {"v5", "B"}, {"v6", "B"}, {"v7", "C"}, {"v2", "D"}
    };

    std::unordered_set<std::string> seen;
    std::unordered_map<std::string, int> tally;

    for (auto& rec : stream) {
        const std::string& voter = rec.first;
        const std::string& cand = rec.second;
        if (seen.count(voter)) {
            std::cout << "Fraud detected: voter " << voter << " voted more than once" << std::endl;
            continue; // do not count fraudulent vote
        }
        seen.insert(voter);
        tally[cand]++;
    }

    std::vector<std::pair<std::string, int>> v(tally.begin(), tally.end());
    std::sort(v.begin(), v.end(), [](const auto& a, const auto& b) {
        if (a.second != b.second) return a.second > b.second; // higher votes first
        return a.first < b.first;                             // tie: candidate id ascending
    });

    std::cout << "Top 3: ";
    int n = std::min((size_t)3, v.size());
    for (int i = 0; i < n; ++i) {
        std::cout << v[i].first << "(" << v[i].second << ")";
        if (i + 1 < n) std::cout << ", ";
    }
    std::cout << std::endl;
    return 0;
}
