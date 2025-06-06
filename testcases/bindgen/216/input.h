template <class a> class Range { a b, c; };
typedef Range<const char *> StringPiece;
typedef Range<char> MutableStringPiece;
Range<char const *> d() {}