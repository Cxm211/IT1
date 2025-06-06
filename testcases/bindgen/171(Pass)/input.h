template <typename> struct a;
template <typename, typename, typename, typename, typename, typename, typename, typename, typename, bool> struct b;
template <typename d, typename c, typename f, typename e, typename h, typename g, typename j, typename i, typename l>
struct b<d, c, f, e, h, g, j, i, l, true> {
  using k = a<l>;
  using n = typename k::n;
  using m = typename c ::o;
  m &p(const n &);
};
template <typename d, typename c, typename f, typename e, typename h, typename g, typename j, typename i, typename l>
auto b<d, c, f, e, h, g, j, i, l, true>::p(const n &) -> m & {}