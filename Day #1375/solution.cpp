// Word circle: backtracking to order all words so each last char == next first char,
// and the last wraps to the first. Time O(n!) worst, Space O(n). (n small)
#include <bits/stdc++.h>
using namespace std;

bool bt(const vector<string>& words, vector<int>& order, vector<bool>& used) {
    if (order.size() == words.size()) {
        return words[order.back()].back() == words[order.front()].front();
    }
    char last = words[order.back()].back();
    for (size_t i = 0; i < words.size(); i++) {
        if (!used[i] && words[i].front() == last) {
            used[i] = true; order.push_back(i);
            if (bt(words, order, used)) return true;
            order.pop_back(); used[i] = false;
        }
    }
    return false;
}

vector<int> circle(const vector<string>& words) {
    vector<bool> used(words.size(), false);
    vector<int> order = {0};
    used[0] = true;
    if (bt(words, order, used)) return order;
    return {};
}

int main() {
    vector<string> words = {"chair", "height", "racket", "touch", "tunic"};
    auto order = circle(words);
    if (order.empty()) cout << "Cannot form a circle\n";
    else {
        for (int idx : order) cout << words[idx] << " --> ";
        cout << words[order[0]] << "\n";
    }
    return 0;
}
