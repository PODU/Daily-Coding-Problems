// Alien dictionary: build edges from first differing char of adjacent words,
// then Kahn's BFS topological sort. Time O(C + V + E), Space O(V + E).
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
    std::map<char, std::set<char>> graph;
    std::map<char, int> indeg;
    for (const auto& w : words)
        for (char c : w) {
            graph[c];
            indeg[c];
        }
    for (size_t i = 0; i + 1 < words.size(); ++i) {
        const std::string& a = words[i];
        const std::string& b = words[i + 1];
        size_t n = std::min(a.size(), b.size());
        for (size_t j = 0; j < n; ++j) {
            if (a[j] != b[j]) {
                if (!graph[a[j]].count(b[j])) {
                    graph[a[j]].insert(b[j]);
                    indeg[b[j]]++;
                }
                break;
            }
        }
    }
    std::set<char> queue;
    for (auto& kv : indeg)
        if (kv.second == 0) queue.insert(kv.first);
    std::vector<char> order;
    while (!queue.empty()) {
        char c = *queue.begin();
        queue.erase(queue.begin());
        order.push_back(c);
        for (char nxt : graph[c])
            if (--indeg[nxt] == 0) queue.insert(nxt);
    }
    std::cout << "[";
    for (size_t i = 0; i < order.size(); ++i) {
        if (i) std::cout << ", ";
        std::cout << "'" << order[i] << "'";
    }
    std::cout << "]\n";
    return 0;
}
