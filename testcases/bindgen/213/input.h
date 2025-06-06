typedef unsigned char uint8_t;
typedef unsigned uint32_t;

struct Bar {
  uint8_t foo;
  uint32_t ns_attr_notice: 1;
  uint32_t fw_activation_notice: 1;
  uint32_t telemetry_log_notice: 1;
  uint32_t reserved: 21;
};