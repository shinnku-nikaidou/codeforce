#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using i64 = std::int64_t;
using usize = std::size_t;
#define in :
using std::vector;

enum class Sheets {
  Excel,
  RC,
};

// R23C55
// BC23
Sheets find(std::string const &input) {
  if (input.at(0) != 'R')
    return Sheets::Excel;
  if (char c = input.at(1); c >= 'A' && c <= 'Z')
    return Sheets::Excel;
  for (auto c = input.begin() + 2; c < input.end(); ++c) {
    if (*c >= 'A' && *c <= 'Z')
      return Sheets::RC;
  }
  return Sheets::Excel;
}

std::string excel2rc(std::string const &input) {
  std::string::const_iterator mid = input.begin();
  for (; *mid >= 'A' && *mid <= 'Z'; ++mid) {
  };
  i64 c{}, r{};
  for (auto i = input.begin(); i != mid; ++i) {
    c *= 26;
    c += static_cast<i64>(*i - '@');
  }
  for (auto i = mid; i != input.end(); ++i) {
    r *= 10;
    r += static_cast<i64>(*i - '0');
  }
  return "R" + std::to_string(r) + "C" + std::to_string(c);
}

std::string rc2excel(std::string const &input) {
  std::string::const_iterator R = input.begin();
  std::string::const_iterator C = R + 1;
  for (; *C != 'C'; ++C) {
  };
  std::string _tmp = std::string(C + 1, input.end());
  i64 r{std::stoi(_tmp)};

  std::string l = {};
  while (r != 0) {
    char a = r % 26 + '@';
    if (a == '@') {
        l.push_back('Z');
        r /= 26;
        r -= 1;
        continue;
    }
    l.push_back(a);
    r /= 26;
  }
  std::reverse(l.begin(), l.end());
  return l + std::string(R + 1, C);
}

int main() {
  std::ios::sync_with_stdio(false);
  usize n;
  std::cin >> n;
  for (usize i = 0; i < n; ++i) {
    std::string input{};
    std::cin >> input;
    Sheets label = find(input);
    switch (label) {
    case Sheets::Excel:
      std::cout << excel2rc(input) << std::endl;
      break;
    case Sheets::RC:
      std::cout << rc2excel(input) << std::endl;
      break;
    }
  }
}

// B. Spreadsheets
// time limit per test10 seconds
// memory limit per test64 megabytes
// inputstandard input
// outputstandard output
// In the popular spreadsheets systems (for example, in Excel) the following
// numeration of columns is used. The first column has number A, the second —
// number B, etc. till column 26 that is marked by Z. Then there are two-letter
// numbers: column 27 has number AA, 28 — AB, column 52 is marked by AZ. After
// ZZ there follow three-letter numbers, etc.

// The rows are marked by integer numbers starting with 1. The cell name is the
// concatenation of the column and the row numbers. For example, BC23 is the
// name for the cell that is in column 55, row 23.

// Sometimes another numeration system is used: RXCY, where X and Y are integer
// numbers, showing the column and the row numbers respectfully. For instance,
// R23C55 is the cell from the previous example.

// Your task is to write a program that reads the given sequence of cell
// coordinates and produce each item written according to the rules of another
// numeration system.

// Input
// The first line of the input contains integer number n (1 ≤ n ≤ 105), the
// number of coordinates in the test. Then there follow n lines, each of them
// contains coordinates. All the coordinates are correct, there are no cells
// with the column and/or the row numbers larger than 106 .

// Output
// Write n lines, each line should contain a cell coordinates in the other
// numeration system.
