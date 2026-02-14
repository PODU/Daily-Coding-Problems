// Day 1065: Reverse a directed graph (reverse every edge).
// Approach: iterate all edges u->v, add v->u to reversed adjacency. Time O(V+E), Space O(V+E).
#include <iostream>
#include <map>
#include <string>
#include <vector>

std::map<std::string, std::vector<std::string>>
reverseGraph(const std::map<std::string, std::vector<std::string>>& g) {
    std::map<std::string, std::vector<std::string>> r;
    for (const auto& kv : g) {
        r.emplace(kv.first, std::vector<std::string>{}); // keep isolated nodes
        for (const auto& v : kv.second) r[v].push_back(kv.first);
    }
    return r;
}

int main() {
    // A -> B -> C
    std::map<std::string, std::vector<std::string>> g = {
        {"A", {"B"}}, {"B", {"C"}}, {"C", {}}};
    auto r = reverseGraph(g);
    // Reversed: B -> A, C -> B  ("A <- B <- C")
    std::cout << "A <- B <- C\n";
    for (const auto& kv : r)
        for (const auto& v : kv.second)
            std::cout << kv.first << " -> " << v << "\n";
    return 0;
}
