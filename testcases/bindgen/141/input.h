namespace std
{
  template < class > class fbstring_core;
}
typedef char uint8_t;
namespace std
{
  template < class > class fbstring_core
  {
    typedef uint8_t category_type;
    enum Category:category_type;
  };
}