#[non_exhaustive]
pub struct Const;

#[allow(unused)]
impl Const {
    pub const GROUP_ANY: &str = "tape";
    pub const GROUP_ARCH: &str = "storage";
    pub const OS_RELEASE_FILE: &str = "/etc/os-release";
    pub const OS_GROUPS_FILE: &str = "/etc/group";
}
