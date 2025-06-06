pub type shared_info_t = shared_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_info {
    pub magic: [::std::os::raw::c_char; 32usize],
    pub nr_pages: ::std::os::raw::c_ulong,
    pub shared_info: ::std::os::raw::c_ulong,
    pub flags: u32,
    pub store_mfn: xen_pfn_t,
    pub store_evtchn: u32,
    pub console: start_info__bindgen_ty_1,
    pub pt_base: ::std::os::raw::c_ulong,
    pub nr_pt_frames: ::std::os::raw::c_ulong,
    pub mfn_list: ::std::os::raw::c_ulong,
    pub mod_start: ::std::os::raw::c_ulong,
    pub mod_len: ::std::os::raw::c_ulong,
    pub cmd_line: [i8; 1024usize],
    pub first_p2m_pfn: ::std::os::raw::c_ulong,
    pub nr_p2m_frames: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Copy, Clone)]; 2usize ] , }#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct start_info__bindgen_ty_1__bindgen_ty_1 {
    pub mfn: xen_pfn_t,
    pub evtchn: u32,
}