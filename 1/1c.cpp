#include <algorithm>
#include <array>
#include <cmath>
#include <complex>
#include <cstdio>
#include <iostream>
#include <iomanip>
#include <string>

using i64 = std::int64_t;
using f128 = long double;
using usize = std::size_t;
#define in :
using std::array;
using namespace std::complex_literals;
constexpr f128 pi = 3.14159265358979323846;

std::complex<f128> circumcircle(std::complex<f128> a, std::complex<f128> b,
                               std::complex<f128> c) {
  std::complex<f128> denominator{}, numerator{};
  auto _a = std::conj(a);
  auto _b = std::conj(b);
  auto _c = std::conj(c);
  numerator = a * b * _b + b * c * _c + c * a * _a - b * a * _a - c * b * _b -
              a * c * _c;
  denominator = a * _b + b * _c + c * _a - a * _c - b * _a - c * _b;
  return numerator / denominator;
}

usize findpolygon(f128 theta1, f128 theta2) {
  theta1 = std::abs(theta1);
  theta2 = std::abs(theta2);
  for (usize i = 3;; ++i) {
    f128 t1 = theta1 * i / (2 * pi);
    t1 -= std::floor(t1);
    f128 t2 = theta2 * i / (2 * pi);
    t2 -= std::floor(t2);

    if ((t1 < 0.0001 || (1 - t1) < 0.0001) &&
        (t2 < 0.0001 || (1 - t2) < 0.0001))
      return i;
  }
}

int main() {
  array<std::complex<f128>, 3> p{};
  // std::freopen("../in.txt", "rb", stdin);
  for (usize i = 0; i < 3; i++) {
    f128 re{}, im{};
    std::cin >> re >> im;
    p[i] = {re, im};
  }
  std::complex<f128> x = circumcircle(p[0], p[1], p[2]);
  std::complex<f128> a = p[0] - x;
  std::complex<f128> b = p[1] - x;
  std::complex<f128> c = p[2] - x;
  f128 l = std::abs(a);
  usize n = findpolygon(std::arg(a) - std::arg(b), std::arg(b) - std::arg(c));
//   std::cout << a << std::endl;
//   std::cout << b << std::endl;
//   std::cout << c << std::endl;
//   std::cout << l << std::endl;
//   std::cout << n << std::endl;
  f128 theta = pi / n;
  f128 S = n * l * l * std::cos(theta) * std::sin(theta);
  std::cout << std::setprecision(16) << S << std::endl;
}

// C. Ancient Berland Circus
// time limit per test2 seconds
// memory limit per test64 megabytes
// inputstandard input
// outputstandard output
// Nowadays all circuses in Berland have a round arena with diameter 13 meters,
// but in the past things were different.

// In Ancient Berland arenas in circuses were shaped as a regular (equiangular)
// polygon, the size and the number of angles could vary from one circus to
// another. In each corner of the arena there was a special pillar, and the rope
// strung between the pillars marked the arena edges.

// Recently the scientists from Berland have discovered the remains of the
// ancient circus arena. They found only three pillars, the others were
// destroyed by the time.

// You are given the coordinates of these three pillars.
// Find out what is the smallest area that the arena could have.

// Input
// The input file consists of three lines, each of them contains a pair of
// numbers –– coordinates of the pillar. Any coordinate doesn't exceed 1000 by
// absolute value, and is given with at most six digits after decimal point.

// Output
// Output the smallest possible area of the ancient arena.
// This number should be accurate to at least 6 digits after the decimal point.
// It's guaranteed that the number of angles in the optimal polygon is not
// larger than 100.
