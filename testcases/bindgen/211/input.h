template <bool> struct a;
template <bool b> using c = typename a<b>::d;
namespace seal {
template <typename e, typename = c<e ::f>> class g { e h; };
template <typename e> class IntArray { g<e> h; };
} // namespace seal