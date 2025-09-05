// Day 222: Normalize an absolute path (resolve . and ..).
// Approach: split on '/', push names onto a stack, pop on "..", skip "." and "". Time O(n), Space O(n).
function simplifyPath(path) {
  const st = [];
  for (const tok of path.split("/")) {
    if (tok === "" || tok === ".") continue;
    if (tok === "..") { if (st.length) st.pop(); }
    else st.push(tok);
  }
  if (st.length === 0) return "/";
  return "/" + st.join("/") + "/"; // trailing slash (directory form)
}

console.log(`"${simplifyPath("/usr/bin/../bin/./scripts/../")}"`); // "/usr/bin/"
