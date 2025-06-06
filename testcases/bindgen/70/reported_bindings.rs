#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash)]
pub union mpw_data__bindgen_ty_1
{
	pub last_dseg: *mut mlx5_wqe_data_seg,
	pub inl_data: *mut u8,
	_bindgen_union_align: u64,
	pub _address: u8,
}