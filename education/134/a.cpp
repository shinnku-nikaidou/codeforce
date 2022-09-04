#include <algorithm>
#include <array>
#include <cmath>
#include <complex>
#include <cstdio>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

using i64 = std::int64_t;
using f128 = long double;
using usize = std::size_t;

int main(int argc, char *argv[]) {
//   std::freopen("../../in.txt", "rb", stdin);
  std::ios::sync_with_stdio(false);
  usize n;
  std::cin >> n;
  for (usize _ = 0; _ < n; _++) {
    std::vector<char> v{};
    std::string s1, s2;
    std::cin >> s1 >> s2;
    for (char c : s1 + s2) {
      if (std::find(v.begin(), v.end(), c) == v.end()) {
        v.push_back(c);
      }
    }
    std::cout << v.size() - 1 << std::endl;
  }
  return 0;
}
