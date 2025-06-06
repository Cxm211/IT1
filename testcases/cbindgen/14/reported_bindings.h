enum WGPUBindingType {
  WGPUBindingType_UniformBuffer = 0,
  WGPUBindingType_StorageBuffer = 1,
  WGPUBindingType_ReadonlyStorageBuffer = 2,
  WGPUBindingType_Sampler = 3,
  WGPUBindingType_SampledTexture = 4,
  WGPUBindingType_StorageTexture = 5,

  Sentinel /* this must be last for serialization purposes. */
};