// Day 1841: Top-k most similar website pairs by Jaccard similarity of their visitor sets.
// Time O(W^2 * U) over W websites; Space O(total pairs). W,U small here.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<string, int>> pairs;
    const char* names0[] = {"a","a","a","b","b","c","c","c","c","c","d","d","d","d","e","e","e","e"};
    int users0[] = {1,3,5, 2,6, 1,2,3,4,5, 4,5,6,7, 1,3,5,6};
    for (int i = 0; i < 18; i++) pairs.push_back(make_pair(string(names0[i]), users0[i]));
    int k = 1;

    map<string, set<int>> sites;
    for (size_t i = 0; i < pairs.size(); i++) sites[pairs[i].first].insert(pairs[i].second);

    vector<string> names;
    for (map<string, set<int>>::iterator it = sites.begin(); it != sites.end(); ++it)
        names.push_back(it->first);

    // (similarity, pair) for all unordered website pairs
    vector<tuple<double, string, string>> scored;
    for (size_t i = 0; i < names.size(); i++)
        for (size_t j = i + 1; j < names.size(); j++) {
            set<int>& A = sites[names[i]];
            set<int>& B = sites[names[j]];
            int inter = 0;
            for (set<int>::iterator x = A.begin(); x != A.end(); ++x)
                if (B.count(*x)) inter++;
            int uni = (int)A.size() + (int)B.size() - inter;
            double jac = uni ? (double)inter / uni : 0.0;
            scored.push_back(make_tuple(jac, names[i], names[j]));
        }
    sort(scored.begin(), scored.end(),
         [](const tuple<double, string, string>& a, const tuple<double, string, string>& b) {
             return get<0>(a) > get<0>(b);
         });

    cout << "[";
    for (int i = 0; i < k; i++)
        cout << "('" << get<1>(scored[i]) << "', '" << get<2>(scored[i]) << "')"
             << (i + 1 < k ? ", " : "");
    cout << "]\n";
}
