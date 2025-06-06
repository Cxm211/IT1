// bindgen-flags:  --blacklist-function enumerateObjectsUsingBlock  --blacklist-type FooWithBlock* --generate-block --objc-extern-crate --block-extern-crate  -- -x objective-c -fblocks
// bindgen-osx-only

@interface FooWithBlock<__covariant ObjectType>
- (void)enumerateObjectsUsingBlock:(void (^)(ObjectType obj, int idx))block;
@end
