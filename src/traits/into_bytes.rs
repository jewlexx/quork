//! Convert given values into their representation in memory

/// Convert given values into their representation in memory
pub trait RamBytes {
    /// Convert a given value into it's memory representation in bytes
    fn to_ram_bytes(&self) -> &[u8];

    /// Convert a given value into it's memory representation in bytes
    fn into_ram_bytes(self) -> Vec<u8>;
}

impl<T> RamBytes for T {
    fn to_ram_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const T).cast::<u8>(),
                core::mem::size_of_val(self),
            )
        }
    }

    fn into_ram_bytes(self) -> Vec<u8>
    where
        Self: Sized,
    {
        Self::to_ram_bytes(&self).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad_with_null<T>(vec: &mut Vec<T>, count: usize) {
        for _ in 0..count {
            vec.push(unsafe { std::mem::zeroed() });
        }
    }

    #[test]
    fn byte_slice() {
        let bytes: [u8; 6] = [0, 1, 2, 3, 4, 5];

        assert_eq!(bytes.into_ram_bytes(), bytes);
    }

    // #[test]
    // fn string_slice() {
    // let hello: &str = "Hello, World!";

    // assert_eq!(RamBytes::to_ram_bytes(hello), hello.as_bytes());
    // }

    #[test]
    fn number_struct() {
        #[derive(Debug, Copy, Clone)]
        #[allow(dead_code)]
        struct VecSize {
            // Note that the size of usize is dependent on your architecture
            // Normally, you would use usize, but this test could fail on certain systems because of that fact
            len: u16,
            capacity: u16,
        }

        let size = VecSize {
            len: 10,
            capacity: 128,
        };

        let bytes = [10, 0, 128, 0];

        assert_eq!(size.to_ram_bytes(), bytes);
    }

    // TODO: Fix this test (string slices are weird for this kind of thing)
    // #[test]
    // fn struct_data() {
    //     #[derive(Debug, Copy, Clone)]
    //     struct Person {
    //         first: &'static str,
    //         surname: &'static str,
    //         age: u8,
    //     }

    //     let person = Person {
    //         first: "Juliette",
    //         surname: "Cordor",
    //         age: 19,
    //     };

    //     // Rust analyzer says the struct is 40 bytes wide
    //     // The first 16 bytes are for the first name
    //     //      The first name is 8 bytes, + 3 for the additional string info
    //     //      That leaves 5 null bytes
    //     // The second 16 bytes are for the last name
    //     //      The first name is 6 bytes, + 3 for the additional string info
    //     //      That leaves 7 null bytes
    //     // The last byte is for the age
    //     // The remainder is null
    //     let person_bytes: Vec<u8> = {
    //         let mut v = Vec::with_capacity(40);

    //         v.extend(person.first.as_bytes());
    //         pad_with_null(&mut v, 5);
    //         v.extend(person.surname.as_bytes());
    //         pad_with_null(&mut v, 7);
    //         v.push(person.age);

    //         v
    //     };
    //     assert_eq!(into_bytes(person), person_bytes);
    // }

    // #[test]
    // fn packed_struct_data() {
    //     #[derive(Debug, Copy, Clone)]
    //     #[repr(packed)]
    //     struct Person {
    //         first: &'static str,
    //         surname: &'static str,
    //         age: u8,
    //     }

    //     let person = Person {
    //         first: "Juliette",
    //         surname: "Cordor",
    //         age: 19,
    //     };

    //     // Rust analyzer says the struct is 40 bytes wide
    //     // The first 16 bytes are for the first name
    //     // The second 16 bytes are for the last name
    //     // The last byte is for the age
    //     let person_bytes: Vec<u8> = {
    //         let mut v = Vec::with_capacity(33);

    //         v.extend(person.first.as_bytes());
    //         v.extend(person.surname.as_bytes());
    //         v.push(person.age);

    //         v
    //     };

    //     assert_eq!(into_bytes(person), person_bytes);
    // }
}
