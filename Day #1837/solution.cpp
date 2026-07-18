// Day 1837: Stable marriage via Gale-Shapley (men propose). Always yields a stable matching.
// Time O(N^2), Space O(N^2) for preference/ranking tables.
#include <bits/stdc++.h>
using namespace std;

map<string, string> stableMatch(
        const map<string, vector<string>>& guyPref,
        const map<string, vector<string>>& galPref) {
    // gal -> rank map for O(1) preference comparison
    map<string, map<string, int>> galRank;
    for (map<string, vector<string>>::const_iterator it = galPref.begin(); it != galPref.end(); ++it) {
        const vector<string>& pref = it->second;
        for (int i = 0; i < (int)pref.size(); i++) galRank[it->first][pref[i]] = i;
    }

    map<string, int> next;                 // next gal index each guy will propose to
    map<string, string> galPartner;        // gal -> current guy (empty if free)
    deque<string> freeGuys;
    for (map<string, vector<string>>::const_iterator it = guyPref.begin(); it != guyPref.end(); ++it) {
        freeGuys.push_back(it->first);
        next[it->first] = 0;
    }

    while (!freeGuys.empty()) {
        string guy = freeGuys.front(); freeGuys.pop_front();
        const string& gal = guyPref.at(guy)[next[guy]++];
        if (!galPartner.count(gal)) {
            galPartner[gal] = guy;
        } else {
            const string cur = galPartner[gal];
            if (galRank[gal][guy] < galRank[gal][cur]) {
                galPartner[gal] = guy;     // gal prefers new guy
                freeGuys.push_back(cur);
            } else {
                freeGuys.push_back(guy);   // gal keeps current
            }
        }
    }
    map<string, string> guyPartner;
    for (map<string, string>::iterator it = galPartner.begin(); it != galPartner.end(); ++it)
        guyPartner[it->second] = it->first;
    return guyPartner;
}

int main() {
    map<string, vector<string>> guy;
    guy["andrew"]  = {"caroline", "abigail", "betty"};
    guy["bill"]    = {"caroline", "betty", "abigail"};
    guy["chester"] = {"betty", "caroline", "abigail"};
    map<string, vector<string>> gal;
    gal["abigail"]  = {"andrew", "bill", "chester"};
    gal["betty"]    = {"bill", "andrew", "chester"};
    gal["caroline"] = {"bill", "chester", "andrew"};

    map<string, string> match = stableMatch(guy, gal);
    for (map<string, string>::iterator it = match.begin(); it != match.end(); ++it)
        cout << it->first << " -> " << it->second << "\n";
}
