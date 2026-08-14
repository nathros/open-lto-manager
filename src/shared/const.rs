#[non_exhaustive]
pub struct Const;

#[allow(unused)]
impl Const {
    pub const GROUP_ANY: &str = "tape";
    pub const GROUP_ARCH: &str = "storage";
    pub const OS_RELEASE_FILE: &str = "/etc/os-release";
    pub const OS_GROUPS_FILE: &str = "/etc/group";

    pub const CODE_39_BARCODE_LEN: usize = 10; // Full length of LTO barcode (including start and end '*')
    // LTO useable length = len(10) -len(2)A -len(2)B = 6
    // A = start(*) and end(*), B = LTO designation
    pub const CODE_39_LTO_MAIN_LEN: usize = 8; // Full length of LTO barcode (excluding start and end '*')
    pub const CODE_39_LTO_USABLE_LEN: usize = 6; // Length of LTO barcode excluding tape designation eg, L7
    pub const CODE_39_LTO_DESIGNATION_LEN: usize = 2;
    pub const BARCODE_VALID_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 -$%./+*";
}
