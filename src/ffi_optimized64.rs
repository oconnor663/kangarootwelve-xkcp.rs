/* automatically generated by rust-bindgen 0.60.1 */

pub type __uint8_t = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KangarooTwelve_FStruct {
    pub state: [u8; 200usize],
    pub byteIOIndex: u8,
    pub squeezing: u8,
}
#[test]
fn bindgen_test_layout_KangarooTwelve_FStruct() {
    assert_eq!(
        ::std::mem::size_of::<KangarooTwelve_FStruct>(),
        202usize,
        concat!("Size of: ", stringify!(KangarooTwelve_FStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<KangarooTwelve_FStruct>(),
        1usize,
        concat!("Alignment of ", stringify!(KangarooTwelve_FStruct))
    );
    fn test_field_state() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_FStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_FStruct),
                "::",
                stringify!(state)
            )
        );
    }
    test_field_state();
    fn test_field_byteIOIndex() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_FStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).byteIOIndex) as usize - ptr as usize
            },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_FStruct),
                "::",
                stringify!(byteIOIndex)
            )
        );
    }
    test_field_byteIOIndex();
    fn test_field_squeezing() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_FStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).squeezing) as usize - ptr as usize
            },
            201usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_FStruct),
                "::",
                stringify!(squeezing)
            )
        );
    }
    test_field_squeezing();
}
pub type KangarooTwelve_F = KangarooTwelve_FStruct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KangarooTwelve_InstanceStruct {
    pub queueNode: KangarooTwelve_F,
    pub __bindgen_padding_0: [u8; 6usize],
    pub finalNode: KangarooTwelve_F,
    pub fixedOutputLength: usize,
    pub blockNumber: usize,
    pub queueAbsorbedLen: ::std::os::raw::c_uint,
    pub phase: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_KangarooTwelve_InstanceStruct() {
    assert_eq!(
        ::std::mem::size_of::<KangarooTwelve_InstanceStruct>(),
        440usize,
        concat!("Size of: ", stringify!(KangarooTwelve_InstanceStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<KangarooTwelve_InstanceStruct>(),
        8usize,
        concat!("Alignment of ", stringify!(KangarooTwelve_InstanceStruct))
    );
    fn test_field_queueNode() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_InstanceStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).queueNode) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_InstanceStruct),
                "::",
                stringify!(queueNode)
            )
        );
    }
    test_field_queueNode();
    fn test_field_finalNode() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_InstanceStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).finalNode) as usize - ptr as usize
            },
            208usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_InstanceStruct),
                "::",
                stringify!(finalNode)
            )
        );
    }
    test_field_finalNode();
    fn test_field_fixedOutputLength() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_InstanceStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fixedOutputLength) as usize - ptr as usize
            },
            416usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_InstanceStruct),
                "::",
                stringify!(fixedOutputLength)
            )
        );
    }
    test_field_fixedOutputLength();
    fn test_field_blockNumber() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_InstanceStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).blockNumber) as usize - ptr as usize
            },
            424usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_InstanceStruct),
                "::",
                stringify!(blockNumber)
            )
        );
    }
    test_field_blockNumber();
    fn test_field_queueAbsorbedLen() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_InstanceStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).queueAbsorbedLen) as usize - ptr as usize
            },
            432usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_InstanceStruct),
                "::",
                stringify!(queueAbsorbedLen)
            )
        );
    }
    test_field_queueAbsorbedLen();
    fn test_field_phase() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<KangarooTwelve_InstanceStruct>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).phase) as usize - ptr as usize
            },
            436usize,
            concat!(
                "Offset of field: ",
                stringify!(KangarooTwelve_InstanceStruct),
                "::",
                stringify!(phase)
            )
        );
    }
    test_field_phase();
}
pub type KangarooTwelve_Instance = KangarooTwelve_InstanceStruct;
extern "C" {
    #[doc = " Function to initialize a KangarooTwelve instance."]
    #[doc = " @param  ktInstance      Pointer to the instance to be initialized."]
    #[doc = " @param  outputByteLen   The desired number of output bytes,"]
    #[doc = "                         or 0 for an arbitrarily-long output."]
    #[doc = " @return 0 if successful, 1 otherwise."]
    pub fn KangarooTwelve_Initialize(
        ktInstance: *mut KangarooTwelve_Instance,
        outputByteLen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Function to give input data to be absorbed."]
    #[doc = " @param  ktInstance      Pointer to the instance initialized by KangarooTwelve_Initialize()."]
    #[doc = " @param  input           Pointer to the input message data (M)."]
    #[doc = " @param  inputByteLen    The number of bytes provided in the input message data."]
    #[doc = " @return 0 if successful, 1 otherwise."]
    pub fn KangarooTwelve_Update(
        ktInstance: *mut KangarooTwelve_Instance,
        input: *const ::std::os::raw::c_uchar,
        inputByteLen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Function to call after all the input message has been input, and to get"]
    #[doc = " output bytes if the length was specified when calling KangarooTwelve_Initialize()."]
    #[doc = " @param  ktInstance      Pointer to the hash instance initialized by KangarooTwelve_Initialize()."]
    #[doc = " If @a outputByteLen was not 0 in the call to KangarooTwelve_Initialize(), the number of"]
    #[doc = "     output bytes is equal to @a outputByteLen."]
    #[doc = " If @a outputByteLen was 0 in the call to KangarooTwelve_Initialize(), the output bytes"]
    #[doc = "     must be extracted using the KangarooTwelve_Squeeze() function."]
    #[doc = " @param  output          Pointer to the buffer where to store the output data."]
    #[doc = " @param  customization   Pointer to the customization string (C)."]
    #[doc = " @param  customByteLen   The length of the customization string in bytes."]
    #[doc = " @return 0 if successful, 1 otherwise."]
    pub fn KangarooTwelve_Final(
        ktInstance: *mut KangarooTwelve_Instance,
        output: *mut ::std::os::raw::c_uchar,
        customization: *const ::std::os::raw::c_uchar,
        customByteLen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Function to squeeze output data."]
    #[doc = " @param  ktInstance     Pointer to the hash instance initialized by KangarooTwelve_Initialize()."]
    #[doc = " @param  data           Pointer to the buffer where to store the output data."]
    #[doc = " @param  outputByteLen  The number of output bytes desired."]
    #[doc = " @pre    KangarooTwelve_Final() must have been already called."]
    #[doc = " @return 0 if successful, 1 otherwise."]
    pub fn KangarooTwelve_Squeeze(
        ktInstance: *mut KangarooTwelve_Instance,
        output: *mut ::std::os::raw::c_uchar,
        outputByteLen: usize,
    ) -> ::std::os::raw::c_int;
}
