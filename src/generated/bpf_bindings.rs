/* automatically generated by rust-bindgen 0.55.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const BPF_LD: u32 = 0;
pub const BPF_LDX: u32 = 1;
pub const BPF_ST: u32 = 2;
pub const BPF_STX: u32 = 3;
pub const BPF_ALU: u32 = 4;
pub const BPF_W: u32 = 0;
pub const BPF_H: u32 = 8;
pub const BPF_B: u32 = 16;
pub const BPF_K: u32 = 0;
pub const BPF_ALU64: u32 = 7;
pub const BPF_DW: u32 = 24;
pub const BPF_PSEUDO_MAP_FD: u32 = 1;
pub const BPF_PSEUDO_MAP_VALUE: u32 = 2;
pub const BPF_PSEUDO_BTF_ID: u32 = 3;
pub const BPF_PSEUDO_CALL: u32 = 1;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_insn {
    pub code: __u8,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub off: __s16,
    pub imm: __s32,
}
impl bpf_insn {
    #[inline]
    pub fn dst_reg(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_dst_reg(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn src_reg(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_src_reg(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(dst_reg: __u8, src_reg: __u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let dst_reg: u8 = unsafe { ::std::mem::transmute(dst_reg) };
            dst_reg as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let src_reg: u8 = unsafe { ::std::mem::transmute(src_reg) };
            src_reg as u64
        });
        __bindgen_bitfield_unit
    }
}
pub mod bpf_cmd {
    pub type Type = ::std::os::raw::c_uint;
    pub const BPF_MAP_CREATE: Type = 0;
    pub const BPF_MAP_LOOKUP_ELEM: Type = 1;
    pub const BPF_MAP_UPDATE_ELEM: Type = 2;
    pub const BPF_MAP_DELETE_ELEM: Type = 3;
    pub const BPF_MAP_GET_NEXT_KEY: Type = 4;
    pub const BPF_PROG_LOAD: Type = 5;
    pub const BPF_OBJ_PIN: Type = 6;
    pub const BPF_OBJ_GET: Type = 7;
    pub const BPF_PROG_ATTACH: Type = 8;
    pub const BPF_PROG_DETACH: Type = 9;
    pub const BPF_PROG_TEST_RUN: Type = 10;
    pub const BPF_PROG_GET_NEXT_ID: Type = 11;
    pub const BPF_MAP_GET_NEXT_ID: Type = 12;
    pub const BPF_PROG_GET_FD_BY_ID: Type = 13;
    pub const BPF_MAP_GET_FD_BY_ID: Type = 14;
    pub const BPF_OBJ_GET_INFO_BY_FD: Type = 15;
    pub const BPF_PROG_QUERY: Type = 16;
    pub const BPF_RAW_TRACEPOINT_OPEN: Type = 17;
    pub const BPF_BTF_LOAD: Type = 18;
    pub const BPF_BTF_GET_FD_BY_ID: Type = 19;
    pub const BPF_TASK_FD_QUERY: Type = 20;
    pub const BPF_MAP_LOOKUP_AND_DELETE_ELEM: Type = 21;
    pub const BPF_MAP_FREEZE: Type = 22;
    pub const BPF_BTF_GET_NEXT_ID: Type = 23;
    pub const BPF_MAP_LOOKUP_BATCH: Type = 24;
    pub const BPF_MAP_LOOKUP_AND_DELETE_BATCH: Type = 25;
    pub const BPF_MAP_UPDATE_BATCH: Type = 26;
    pub const BPF_MAP_DELETE_BATCH: Type = 27;
    pub const BPF_LINK_CREATE: Type = 28;
    pub const BPF_LINK_UPDATE: Type = 29;
    pub const BPF_LINK_GET_FD_BY_ID: Type = 30;
    pub const BPF_LINK_GET_NEXT_ID: Type = 31;
    pub const BPF_ENABLE_STATS: Type = 32;
    pub const BPF_ITER_CREATE: Type = 33;
    pub const BPF_LINK_DETACH: Type = 34;
    pub const BPF_PROG_BIND_MAP: Type = 35;
}
pub mod bpf_map_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const BPF_MAP_TYPE_UNSPEC: Type = 0;
    pub const BPF_MAP_TYPE_HASH: Type = 1;
    pub const BPF_MAP_TYPE_ARRAY: Type = 2;
    pub const BPF_MAP_TYPE_PROG_ARRAY: Type = 3;
    pub const BPF_MAP_TYPE_PERF_EVENT_ARRAY: Type = 4;
    pub const BPF_MAP_TYPE_PERCPU_HASH: Type = 5;
    pub const BPF_MAP_TYPE_PERCPU_ARRAY: Type = 6;
    pub const BPF_MAP_TYPE_STACK_TRACE: Type = 7;
    pub const BPF_MAP_TYPE_CGROUP_ARRAY: Type = 8;
    pub const BPF_MAP_TYPE_LRU_HASH: Type = 9;
    pub const BPF_MAP_TYPE_LRU_PERCPU_HASH: Type = 10;
    pub const BPF_MAP_TYPE_LPM_TRIE: Type = 11;
    pub const BPF_MAP_TYPE_ARRAY_OF_MAPS: Type = 12;
    pub const BPF_MAP_TYPE_HASH_OF_MAPS: Type = 13;
    pub const BPF_MAP_TYPE_DEVMAP: Type = 14;
    pub const BPF_MAP_TYPE_SOCKMAP: Type = 15;
    pub const BPF_MAP_TYPE_CPUMAP: Type = 16;
    pub const BPF_MAP_TYPE_XSKMAP: Type = 17;
    pub const BPF_MAP_TYPE_SOCKHASH: Type = 18;
    pub const BPF_MAP_TYPE_CGROUP_STORAGE: Type = 19;
    pub const BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: Type = 20;
    pub const BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: Type = 21;
    pub const BPF_MAP_TYPE_QUEUE: Type = 22;
    pub const BPF_MAP_TYPE_STACK: Type = 23;
    pub const BPF_MAP_TYPE_SK_STORAGE: Type = 24;
    pub const BPF_MAP_TYPE_DEVMAP_HASH: Type = 25;
    pub const BPF_MAP_TYPE_STRUCT_OPS: Type = 26;
    pub const BPF_MAP_TYPE_RINGBUF: Type = 27;
    pub const BPF_MAP_TYPE_INODE_STORAGE: Type = 28;
    pub const BPF_MAP_TYPE_TASK_STORAGE: Type = 29;
}
pub mod bpf_prog_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const BPF_PROG_TYPE_UNSPEC: Type = 0;
    pub const BPF_PROG_TYPE_SOCKET_FILTER: Type = 1;
    pub const BPF_PROG_TYPE_KPROBE: Type = 2;
    pub const BPF_PROG_TYPE_SCHED_CLS: Type = 3;
    pub const BPF_PROG_TYPE_SCHED_ACT: Type = 4;
    pub const BPF_PROG_TYPE_TRACEPOINT: Type = 5;
    pub const BPF_PROG_TYPE_XDP: Type = 6;
    pub const BPF_PROG_TYPE_PERF_EVENT: Type = 7;
    pub const BPF_PROG_TYPE_CGROUP_SKB: Type = 8;
    pub const BPF_PROG_TYPE_CGROUP_SOCK: Type = 9;
    pub const BPF_PROG_TYPE_LWT_IN: Type = 10;
    pub const BPF_PROG_TYPE_LWT_OUT: Type = 11;
    pub const BPF_PROG_TYPE_LWT_XMIT: Type = 12;
    pub const BPF_PROG_TYPE_SOCK_OPS: Type = 13;
    pub const BPF_PROG_TYPE_SK_SKB: Type = 14;
    pub const BPF_PROG_TYPE_CGROUP_DEVICE: Type = 15;
    pub const BPF_PROG_TYPE_SK_MSG: Type = 16;
    pub const BPF_PROG_TYPE_RAW_TRACEPOINT: Type = 17;
    pub const BPF_PROG_TYPE_CGROUP_SOCK_ADDR: Type = 18;
    pub const BPF_PROG_TYPE_LWT_SEG6LOCAL: Type = 19;
    pub const BPF_PROG_TYPE_LIRC_MODE2: Type = 20;
    pub const BPF_PROG_TYPE_SK_REUSEPORT: Type = 21;
    pub const BPF_PROG_TYPE_FLOW_DISSECTOR: Type = 22;
    pub const BPF_PROG_TYPE_CGROUP_SYSCTL: Type = 23;
    pub const BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE: Type = 24;
    pub const BPF_PROG_TYPE_CGROUP_SOCKOPT: Type = 25;
    pub const BPF_PROG_TYPE_TRACING: Type = 26;
    pub const BPF_PROG_TYPE_STRUCT_OPS: Type = 27;
    pub const BPF_PROG_TYPE_EXT: Type = 28;
    pub const BPF_PROG_TYPE_LSM: Type = 29;
    pub const BPF_PROG_TYPE_SK_LOOKUP: Type = 30;
}
pub mod bpf_attach_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const BPF_CGROUP_INET_INGRESS: Type = 0;
    pub const BPF_CGROUP_INET_EGRESS: Type = 1;
    pub const BPF_CGROUP_INET_SOCK_CREATE: Type = 2;
    pub const BPF_CGROUP_SOCK_OPS: Type = 3;
    pub const BPF_SK_SKB_STREAM_PARSER: Type = 4;
    pub const BPF_SK_SKB_STREAM_VERDICT: Type = 5;
    pub const BPF_CGROUP_DEVICE: Type = 6;
    pub const BPF_SK_MSG_VERDICT: Type = 7;
    pub const BPF_CGROUP_INET4_BIND: Type = 8;
    pub const BPF_CGROUP_INET6_BIND: Type = 9;
    pub const BPF_CGROUP_INET4_CONNECT: Type = 10;
    pub const BPF_CGROUP_INET6_CONNECT: Type = 11;
    pub const BPF_CGROUP_INET4_POST_BIND: Type = 12;
    pub const BPF_CGROUP_INET6_POST_BIND: Type = 13;
    pub const BPF_CGROUP_UDP4_SENDMSG: Type = 14;
    pub const BPF_CGROUP_UDP6_SENDMSG: Type = 15;
    pub const BPF_LIRC_MODE2: Type = 16;
    pub const BPF_FLOW_DISSECTOR: Type = 17;
    pub const BPF_CGROUP_SYSCTL: Type = 18;
    pub const BPF_CGROUP_UDP4_RECVMSG: Type = 19;
    pub const BPF_CGROUP_UDP6_RECVMSG: Type = 20;
    pub const BPF_CGROUP_GETSOCKOPT: Type = 21;
    pub const BPF_CGROUP_SETSOCKOPT: Type = 22;
    pub const BPF_TRACE_RAW_TP: Type = 23;
    pub const BPF_TRACE_FENTRY: Type = 24;
    pub const BPF_TRACE_FEXIT: Type = 25;
    pub const BPF_MODIFY_RETURN: Type = 26;
    pub const BPF_LSM_MAC: Type = 27;
    pub const BPF_TRACE_ITER: Type = 28;
    pub const BPF_CGROUP_INET4_GETPEERNAME: Type = 29;
    pub const BPF_CGROUP_INET6_GETPEERNAME: Type = 30;
    pub const BPF_CGROUP_INET4_GETSOCKNAME: Type = 31;
    pub const BPF_CGROUP_INET6_GETSOCKNAME: Type = 32;
    pub const BPF_XDP_DEVMAP: Type = 33;
    pub const BPF_CGROUP_INET_SOCK_RELEASE: Type = 34;
    pub const BPF_XDP_CPUMAP: Type = 35;
    pub const BPF_SK_LOOKUP: Type = 36;
    pub const BPF_XDP: Type = 37;
    pub const __MAX_BPF_ATTACH_TYPE: Type = 38;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr {
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_1,
    pub __bindgen_anon_2: bpf_attr__bindgen_ty_2,
    pub batch: bpf_attr__bindgen_ty_3,
    pub __bindgen_anon_3: bpf_attr__bindgen_ty_4,
    pub __bindgen_anon_4: bpf_attr__bindgen_ty_5,
    pub __bindgen_anon_5: bpf_attr__bindgen_ty_6,
    pub test: bpf_attr__bindgen_ty_7,
    pub __bindgen_anon_6: bpf_attr__bindgen_ty_8,
    pub info: bpf_attr__bindgen_ty_9,
    pub query: bpf_attr__bindgen_ty_10,
    pub raw_tracepoint: bpf_attr__bindgen_ty_11,
    pub __bindgen_anon_7: bpf_attr__bindgen_ty_12,
    pub task_fd_query: bpf_attr__bindgen_ty_13,
    pub link_create: bpf_attr__bindgen_ty_14,
    pub link_update: bpf_attr__bindgen_ty_15,
    pub link_detach: bpf_attr__bindgen_ty_16,
    pub enable_stats: bpf_attr__bindgen_ty_17,
    pub iter_create: bpf_attr__bindgen_ty_18,
    pub prog_bind_map: bpf_attr__bindgen_ty_19,
    _bindgen_union_align: [u64; 15usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_1 {
    pub map_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub inner_map_fd: __u32,
    pub numa_node: __u32,
    pub map_name: [::std::os::raw::c_char; 16usize],
    pub map_ifindex: __u32,
    pub btf_fd: __u32,
    pub btf_key_type_id: __u32,
    pub btf_value_type_id: __u32,
    pub btf_vmlinux_value_type_id: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr__bindgen_ty_2 {
    pub map_fd: __u32,
    pub key: __u64,
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_2__bindgen_ty_1,
    pub flags: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr__bindgen_ty_2__bindgen_ty_1 {
    pub value: __u64,
    pub next_key: __u64,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_3 {
    pub in_batch: __u64,
    pub out_batch: __u64,
    pub keys: __u64,
    pub values: __u64,
    pub count: __u32,
    pub map_fd: __u32,
    pub elem_flags: __u64,
    pub flags: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr__bindgen_ty_4 {
    pub prog_type: __u32,
    pub insn_cnt: __u32,
    pub insns: __u64,
    pub license: __u64,
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_buf: __u64,
    pub kern_version: __u32,
    pub prog_flags: __u32,
    pub prog_name: [::std::os::raw::c_char; 16usize],
    pub prog_ifindex: __u32,
    pub expected_attach_type: __u32,
    pub prog_btf_fd: __u32,
    pub func_info_rec_size: __u32,
    pub func_info: __u64,
    pub func_info_cnt: __u32,
    pub line_info_rec_size: __u32,
    pub line_info: __u64,
    pub line_info_cnt: __u32,
    pub attach_btf_id: __u32,
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_4__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr__bindgen_ty_4__bindgen_ty_1 {
    pub attach_prog_fd: __u32,
    pub attach_btf_obj_fd: __u32,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_5 {
    pub pathname: __u64,
    pub bpf_fd: __u32,
    pub file_flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_6 {
    pub target_fd: __u32,
    pub attach_bpf_fd: __u32,
    pub attach_type: __u32,
    pub attach_flags: __u32,
    pub replace_bpf_fd: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_7 {
    pub prog_fd: __u32,
    pub retval: __u32,
    pub data_size_in: __u32,
    pub data_size_out: __u32,
    pub data_in: __u64,
    pub data_out: __u64,
    pub repeat: __u32,
    pub duration: __u32,
    pub ctx_size_in: __u32,
    pub ctx_size_out: __u32,
    pub ctx_in: __u64,
    pub ctx_out: __u64,
    pub flags: __u32,
    pub cpu: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr__bindgen_ty_8 {
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_8__bindgen_ty_1,
    pub next_id: __u32,
    pub open_flags: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr__bindgen_ty_8__bindgen_ty_1 {
    pub start_id: __u32,
    pub prog_id: __u32,
    pub map_id: __u32,
    pub btf_id: __u32,
    pub link_id: __u32,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_9 {
    pub bpf_fd: __u32,
    pub info_len: __u32,
    pub info: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_10 {
    pub target_fd: __u32,
    pub attach_type: __u32,
    pub query_flags: __u32,
    pub attach_flags: __u32,
    pub prog_ids: __u64,
    pub prog_cnt: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_11 {
    pub name: __u64,
    pub prog_fd: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_12 {
    pub btf: __u64,
    pub btf_log_buf: __u64,
    pub btf_size: __u32,
    pub btf_log_size: __u32,
    pub btf_log_level: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_13 {
    pub pid: __u32,
    pub fd: __u32,
    pub flags: __u32,
    pub buf_len: __u32,
    pub buf: __u64,
    pub prog_id: __u32,
    pub fd_type: __u32,
    pub probe_offset: __u64,
    pub probe_addr: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr__bindgen_ty_14 {
    pub prog_fd: __u32,
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_14__bindgen_ty_1,
    pub attach_type: __u32,
    pub flags: __u32,
    pub __bindgen_anon_2: bpf_attr__bindgen_ty_14__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr__bindgen_ty_14__bindgen_ty_1 {
    pub target_fd: __u32,
    pub target_ifindex: __u32,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr__bindgen_ty_14__bindgen_ty_2 {
    pub target_btf_id: __u32,
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_14__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: [u64; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_14__bindgen_ty_2__bindgen_ty_1 {
    pub iter_info: __u64,
    pub iter_info_len: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_15 {
    pub link_fd: __u32,
    pub new_prog_fd: __u32,
    pub flags: __u32,
    pub old_prog_fd: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_16 {
    pub link_fd: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_17 {
    pub type_: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_18 {
    pub link_fd: __u32,
    pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bpf_attr__bindgen_ty_19 {
    pub prog_fd: __u32,
    pub map_fd: __u32,
    pub flags: __u32,
}
