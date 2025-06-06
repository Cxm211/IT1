[...]
#[repr(C, packed)]
#[derive(Debug, Copy)]
pub struct virtio_net_ctrl_mac {
    pub entries: __virtio32,
    pub macs: __IncompleteArrayField<[__u8; 6usize]>,
}
#[test]
fn bindgen_test_layout_virtio_net_ctrl_mac() {
    assert_eq!(::std::mem::size_of::<virtio_net_ctrl_mac>() , 4usize , concat
               ! ( "Size of: " , stringify ! ( virtio_net_ctrl_mac ) ));
    assert_eq! (::std::mem::align_of::<virtio_net_ctrl_mac>() , 1usize ,
                concat ! (
                "Alignment of " , stringify ! ( virtio_net_ctrl_mac ) ));
}
[...]