#pragma once

#include "sux/bits/WordDynRankSel.hpp"
#include "sux/util/FenwickByteL.hpp"
#include "sux/util/Vector.hpp"

using namespace std;
using namespace sux;
using namespace sux::util;
using namespace sux::bits;

using BV = Vector<uint64_t>;

// BV merge_bv(BV fst, BV snd);
// BV intersect_bv(BV fst, BV snd);

class RankBV {
private:
  mutable BV bitvect;
  mutable WordDynRankSel<FenwickByteL> bv;

public:
  RankBV(size_t size);
  // RankBV(size_t size, BV &bitvect);
  ~RankBV();

  // Delete copy operators
  RankBV(const RankBV &) = delete;
  RankBV &operator=(const RankBV &) = delete;

  size_t size() const;
  bool get(size_t index) const;
  bool set(size_t index) const;
  bool clear(size_t index) const;
  bool toggle(size_t index) const;
  uint64_t rank(size_t index) const;
  size_t count_ones() const;
  // RankBV merge(RankBV &other);
  // RankBV intersect(RankBV &other);
};

unique_ptr<RankBV> new_rank_bv(size_t size);
