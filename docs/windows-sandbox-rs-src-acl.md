# windows-sandbox-rs/src/acl.rs

## 文件作用

操作 Windows ACL（访问控制列表），为指定 SID 添加、撤销文件和设备的访问权限。

## 主要函数和方法

### `pub unsafe fn dacl_has_write_allow_for_sid(p_dacl: *mut ACL, psid: *mut c_void) -> bool`
检查 DACL 是否包含允许指定 SID 写入的 ACE（忽略仅继承的 ACE）

### `pub unsafe fn dacl_effective_allows_write(p_dacl: *mut ACL, psid: *mut c_void) -> bool`
计算 SID 对 DACL 的有效权限，判断写入是否被允许（考虑拒绝 ACE 和顺序）

### `pub unsafe fn add_allow_ace(path: &Path, psid: *mut c_void) -> Result<bool>`
为指定路径添加允许 SID 的 ACE，授予读、写、执行权限

### `pub unsafe fn revoke_ace(path: &Path, psid: *mut c_void)`
撤销指定路径的 SID 权限

### `pub unsafe fn allow_null_device(psid: *mut c_void)`
为指定 SID 授予 NUL 设备的访问权限

## 主要常量

- `INHERIT_ONLY_ACE`: 0x08
- `CONTAINER_INHERIT_ACE`: 0x2
- `OBJECT_INHERIT_ACE`: 0x1
