template <int, typename> struct a {};
enum { b };
namespace JS {
template <typename c> using d = a<b, c>;
template <typename c> class e { d<c> f; };
class AutoIdVector : e<int> {};
}