// Gale-Shapley stable marriage, men propose; free man proposes down his list, women keep better partner.
// Time O(n^2), Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<string> men = {"andrew", "bill", "chester"};
    vector<string> women = {"abigail", "betty", "caroline"};
    map<string, vector<string>> guyPref = {
        {"andrew", {"caroline", "abigail", "betty"}},
        {"bill", {"caroline", "betty", "abigail"}},
        {"chester", {"betty", "caroline", "abigail"}}};
    map<string, vector<string>> galPref = {
        {"abigail", {"andrew", "bill", "chester"}},
        {"betty", {"bill", "andrew", "chester"}},
        {"caroline", {"bill", "chester", "andrew"}}};

    // woman -> rank of each man
    map<string, map<string, int>> wrank;
    for (auto &w : women)
        for (int i = 0; i < (int)galPref[w].size(); i++)
            wrank[w][galPref[w][i]] = i;

    map<string, int> next; // next index in man's pref list
    for (auto &m : men) next[m] = 0;
    map<string, string> partnerOf; // woman -> man
    deque<string> freeMen(men.begin(), men.end());

    while (!freeMen.empty()) {
        string m = freeMen.front(); freeMen.pop_front();
        string w = guyPref[m][next[m]++];
        if (!partnerOf.count(w)) {
            partnerOf[w] = m;
        } else {
            string cur = partnerOf[w];
            if (wrank[w][m] < wrank[w][cur]) {
                partnerOf[w] = m;
                freeMen.push_back(cur);
            } else {
                freeMen.push_back(m);
            }
        }
    }

    map<string, string> manToWoman;
    for (auto &p : partnerOf) manToWoman[p.second] = p.first;
    for (auto &m : men) // men already sorted
        cout << m << " - " << manToWoman[m] << "\n";
    return 0;
}
