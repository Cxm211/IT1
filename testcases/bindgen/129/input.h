template <typename> struct a;
namespace JS {
template <typename T> using b = a<T>;
template <typename T> class Rooted { b<T> c; };
}