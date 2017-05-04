#[derive(Debug, PartialEq)]
pub struct MAC([u8; 6]);

/// MAC incapsulates operations with a MAC address.
///
/// # Examples
///
/// ```
/// use mdhcpd::mac::MAC;
///
/// let m = MAC::from_string("00:11:33:aa:be:ef").unwrap();
/// assert_eq!(m.as_array(), &[0x00, 0x11, 0x33, 0xaa, 0xbe, 0xef]);
/// assert_eq!(format!("{}", m), "00:11:33:aa:be:ef");
///
/// assert_eq!(MAC::from_string("00:11:22"), None);
///
/// assert_eq!(MAC::from_string("00:11:22:33:44:55:66"), None);
///
/// assert_eq!(MAC::from_string("th:is:is:ba:dm:ac"), None);
/// ```
impl MAC {
    pub fn from_string(mac: &str) -> Option<MAC> {
        let mut parsed = [0; 6];
        let mut idx = 0;
        for octet in mac.split(':') {
            if idx == 6 {
                return None;
            }

            match u8::from_str_radix(octet, 16) {
                Ok(val) => parsed[idx] = val,
                Err(_) => return None,
            }

            idx += 1;
        }
        if idx != 6 {
            return None;
        }
        Some(MAC(parsed))
    }

    pub fn from_array(mac: [u8; 6]) -> MAC {
        MAC(mac)
    }

    pub fn as_array(&self) -> &[u8; 6] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl ::std::str::FromStr for MAC {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match MAC::from_string(s) {
            Some(mac) => Ok(mac),
            None => Err(()),
        }
    }
}

impl ::std::fmt::Display for MAC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f,
               "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
               self.0[0],
               self.0[1],
               self.0[2],
               self.0[3],
               self.0[4],
               self.0[5])
    }
}
