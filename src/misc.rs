use std::net::IpAddr;

pub fn is_global(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            !(ip.octets()[0] == 0
                || ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || (ip.octets()[0] == 192 && ip.octets()[1] == 0 && ip.octets()[2] == 0)
                || ip.is_documentation()
                || ip.is_broadcast())
        }
        IpAddr::V6(ip) => {
            !(ip.is_unspecified()
                || ip.is_loopback()
                || matches!(ip.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
                || matches!(ip.segments(), [0x64, 0xff9b, 1, _, _, _, _, _])
                || matches!(ip.segments(), [0x100, 0, 0, 0, _, _, _, _])
                || (matches!(ip.segments(), [0x2001, b, _, _, _, _, _, _] if b < 0x200)
                    && !(u128::from_be_bytes(ip.octets())
                        == 0x2001_0001_0000_0000_0000_0000_0000_0001
                        || u128::from_be_bytes(ip.octets())
                            == 0x2001_0001_0000_0000_0000_0000_0000_0002
                        || matches!(ip.segments(), [0x2001, 3, _, _, _, _, _, _])
                        || matches!(ip.segments(), [0x2001, 4, 0x112, _, _, _, _, _])
                        || matches!(ip.segments(), [0x2001, b, _, _, _, _, _, _] if (0x20..=0x2F).contains(&b)))))
        }
    }
}
