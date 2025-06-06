namespace JS {
template <typename a> class Rooted { a b; };
class AutoValueVector : Rooted<int> {
  using Vec = int;
  using c = Rooted<Vec>;
};
}