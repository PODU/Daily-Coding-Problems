// Lazy bartender = min set cover (NP-hard). Greedy: repeatedly pick drink covering
// most uncovered customers. Time O(D^2*C), Space O(D*C).
function fewestDrinks(preferences) {
    const drinkToCust = new Map();
    const uncovered = new Set(Object.keys(preferences).map(Number));
    for (const [cust, drinks] of Object.entries(preferences)) {
        for (const d of drinks) {
            if (!drinkToCust.has(d)) drinkToCust.set(d, new Set());
            drinkToCust.get(d).add(Number(cust));
        }
    }
    let learned = 0;
    while (uncovered.size > 0) {
        let bestDrink = null, bestCount = 0;
        for (const [drink, custs] of drinkToCust) {
            let cnt = 0;
            for (const c of custs) if (uncovered.has(c)) cnt++;
            if (cnt > bestCount) { bestCount = cnt; bestDrink = drink; }
        }
        if (bestDrink === null) break;
        for (const c of drinkToCust.get(bestDrink)) uncovered.delete(c);
        learned++;
    }
    return learned;
}

function main() {
    const preferences = {0:[0,1,3,6], 1:[1,4,7], 2:[2,4,7,5], 3:[3,2,5], 4:[5,8]};
    console.log(fewestDrinks(preferences));
}

main();
