/**
 * IPv6 tuple
 * Addresses have to be filled by rte_thash_load_v6_addr()
 * ports/sctp_tag have to be CPU byte order
 */
 #[repr(C)]
 #[derive(Debug, Copy)]
 pub struct rte_ipv6_tuple {
     pub src_addr: [u8; 16usize],
     pub dst_addr: [u8; 16usize],
     pub __bindgen_anon_1: rte_ipv6_tuple__bindgen_ty_1,
 }
 #[repr(C)]
 #[derive(Debug, Copy)]
 pub struct rte_ipv6_tuple__bindgen_ty_1 {
     pub __bindgen_anon_1: __BindgenUnionField<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>,
     pub sctp_tag: __BindgenUnionField<u32>,
     pub bindgen_union_field: u32,
 }
 #[repr(C)]
 #[derive(Debug, Copy)]
 pub struct rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1 {
     pub dport: u16,
     pub sport: u16,
 }
 #[test]
 fn bindgen_test_layout_rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1() {
     assert_eq!(::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>()
                , 4usize);
     assert_eq!(::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>()
                , 2usize);
 }
 impl Clone for rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1 {
     fn clone(&self) -> Self { *self }
 }
 #[test]
 fn bindgen_test_layout_rte_ipv6_tuple__bindgen_ty_1() {
     assert_eq!(::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1>() ,
                4usize);
     assert_eq!(::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1>() ,
                4usize);
 }
 impl Clone for rte_ipv6_tuple__bindgen_ty_1 {
     fn clone(&self) -> Self { *self }
 }
 #[test]
 fn bindgen_test_layout_rte_ipv6_tuple() {
     assert_eq!(::std::mem::size_of::<rte_ipv6_tuple>() , 36usize);
     assert_eq!(::std::mem::align_of::<rte_ipv6_tuple>() , 4usize);
 }
 impl Clone for rte_ipv6_tuple {
     fn clone(&self) -> Self { *self }
 }
 #[repr(C)]
 #[derive(Debug, Copy)]
 pub struct rte_thash_tuple {
     pub v4: __BindgenUnionField<rte_ipv4_tuple>,
     pub v6: __BindgenUnionField<rte_ipv6_tuple>,
     pub bindgen_union_field: [u8; 48usize],
 }
 impl Clone for rte_thash_tuple {
     fn clone(&self) -> Self { *self }
 }