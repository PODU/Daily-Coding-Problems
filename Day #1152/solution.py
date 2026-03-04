# Day 1152: Simplify absolute Unix path.
# Stack of components; '.' ignored, '..' pops. Time O(n), Space O(n).
def simplify(path):
    st = []
    for part in path.split("/"):
        if part == "" or part == ".":
            continue
        if part == "..":
            if st:
                st.pop()
        else:
            st.append(part)
    return "/" + "".join(p + "/" for p in st)  # trailing slash preserved


if __name__ == "__main__":
    print(simplify("/usr/bin/../bin/./scripts/../"))  # /usr/bin/
