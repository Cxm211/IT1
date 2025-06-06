#[repr(C)]
#[derive(Debug, Copy)]
pub struct rte_cryptodev {
    pub dequeue_burst: dequeue_pkt_burst_t,
    pub enqueue_burst: enqueue_pkt_burst_t,
    pub driver: *const rte_cryptodev_driver,
    pub data: *mut rte_cryptodev_data,
    pub dev_ops: *mut rte_cryptodev_ops,
    pub feature_flags: u64,
    pub device: *mut rte_device,
    pub dev_type: rte_cryptodev_type,
    pub link_intr_cbs: rte_cryptodev_cb_list,
    pub _bitfield_1: u8,
    pub __bindgen_padding_0: [u8; 47usize],
}