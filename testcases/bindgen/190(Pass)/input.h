// Comment out this to turn *mut b -> *const b in generated Rust code.
// #define BUG
#if !defined(BUG)
struct b;
#endif
struct a {
	const struct b *x;
}; 