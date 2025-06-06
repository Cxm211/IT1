// test.h
struct virtio_net_ctrl_mac {
        unsigned entries;
        unsigned char macs[][6];
} __attribute__((packed));