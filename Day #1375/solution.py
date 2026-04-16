# Day 1375: Word circle: backtracking to order all words so each last char == next first char,
# and the last wraps to the first. Time O(n!) worst, Space O(n). (n small)


def circle(words):
    n = len(words)
    used = [False] * n
    order = [0]
    used[0] = True

    def bt():
        if len(order) == n:
            return words[order[-1]][-1] == words[order[0]][0]
        last = words[order[-1]][-1]
        for i in range(n):
            if not used[i] and words[i][0] == last:
                used[i] = True
                order.append(i)
                if bt():
                    return True
                order.pop()
                used[i] = False
        return False

    return order if bt() else None


if __name__ == "__main__":
    words = ["chair", "height", "racket", "touch", "tunic"]
    order = circle(words)
    if order is None:
        print("Cannot form a circle")
    else:
        print(" --> ".join(words[i] for i in order) + " --> " + words[order[0]])
