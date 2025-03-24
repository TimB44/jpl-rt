use std::{ffi::c_void, iter::repeat_n, slice, str::from_utf8};

use crate::{fail, to_str};
const WORD_SIZE: usize = 8;

#[no_mangle]
pub extern "C" fn show(type_str: *const i8, data: *const c_void) {
    let mut type_str = TypeStr {
        src: to_str(type_str).as_bytes(),
        cur: 0,
    };

    let data_type = type_str.parse();
    let mut data_str = String::new();
    data_type.write_type(data, &mut data_str);

    println!("{}", data_str);
}

#[no_mangle]
pub extern "C" fn _show(type_str: *const i8, data: *const c_void) {
    show(type_str, data)
}

struct TypeStr<'a> {
    src: &'a [u8],
    cur: usize,
}

enum Type {
    Tuple(Vec<Type>),
    Array(Box<Type>, u8),
    Int,
    Float,
    Void,
    Bool,
}

impl<'a> TypeStr<'a> {
    fn parse(&mut self) -> Type {
        self.skip_bytes(b"(");
        self.skip_spaces();
        let data_type = match self.src.get(self.cur).copied() {
            Some(b'I') => {
                self.skip_bytes(b"IntType");
                Type::Int
            }
            Some(b'F') => {
                self.skip_bytes(b"FloatType");
                Type::Float
            }
            Some(b'B') => {
                self.skip_bytes(b"BoolType");
                Type::Bool
            }
            Some(b'V') => {
                self.skip_bytes(b"VoidType");
                Type::Void
            }
            Some(b'T') => {
                self.skip_bytes(b"TupleType");
                let mut fields = Vec::new();
                loop {
                    self.skip_spaces();
                    match self.src.get(self.cur) {
                        Some(b')') => break Type::Tuple(fields),
                        None => fail!("parse_type_str", "Invalid type string"),
                        _ => (),
                    }

                    fields.push(self.parse());
                }
            }

            Some(b'A') => {
                self.skip_bytes(b"ArrayType");
                self.skip_spaces();
                let element_type = self.parse();
                self.skip_spaces();
                let rank = self.parse_integer();

                Type::Array(Box::new(element_type), rank)
            }
            _ => fail!("parse_type_str", "Invalid type string"),
        };
        self.skip_spaces();
        self.skip_bytes(b")");
        data_type
    }

    fn parse_integer(&mut self) -> u8 {
        let start = self.cur;
        while self.cur < self.src.len() && self.src[self.cur].is_ascii_digit() {
            self.cur += 1;
        }
        let num_str = from_utf8(&self.src[start..self.cur]).unwrap();
        num_str
            .parse()
            .unwrap_or_else(|_| fail!("parse_type_str", "Invalid integer in type string"))
    }

    fn skip_bytes(&mut self, bytes: &[u8]) {
        if !self.src[self.cur..].starts_with(bytes) {
            fail!("parse_type_str", "Invalid type string")
        }

        self.cur += bytes.len();
    }

    fn skip_spaces(&mut self) {
        while let Some(c) = self.src.get(self.cur) {
            if !c.is_ascii_whitespace() {
                break;
            }
            self.cur += 1
        }
    }
}

impl Type {
    /// Returns the size in bytes for this type.
    fn size(&self) -> usize {
        match self {
            Type::Int | Type::Float | Type::Bool | Type::Void => WORD_SIZE,
            Type::Tuple(fields) => fields.iter().map(|f| f.size()).sum(),
            Type::Array(_, rank) => WORD_SIZE + WORD_SIZE * *rank as usize,
        }
    }

    fn write_type(&self, data: *const c_void, out: &mut String) {
        match self {
            Type::Int => {
                let value = unsafe { *(data as *const i64) };
                out.push_str(&value.to_string());
            }
            Type::Float => {
                // Read a 64-bit floating-point value.
                let value = unsafe { *(data as *const f64) };
                out.push_str(&value.to_string());
            }
            Type::Bool => {
                // Read a 64-bit value and treat zero as false and nonzero as true.
                let value = unsafe { *(data as *const u64) };
                out.push_str(if value != 0 { "true" } else { "false" });
            }
            Type::Void => {
                // For Void, just print "void" (or you could choose to print nothing).
                out.push_str("void");
            }
            Type::Tuple(fields) => {
                out.push('{');
                // Convert data to a byte pointer for offset arithmetic.
                let mut offset = data as *const u8;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    field.write_type(offset as *const c_void, out);
                    // Advance the pointer by the size of the field.
                    unsafe {
                        offset = offset.add(field.size());
                    }
                }
                out.push('}');
            }
            Type::Array(element, rank) => {
                let dims = unsafe { slice::from_raw_parts(data as *const u64, *rank as usize) };
                let arr_ptr = unsafe { *(data.add(*rank as usize * 8) as *const *const c_void) };
                let num_items = dims.iter().product();

                let element_size = element.size();

                out.push('[');
                for i in 0..num_items {
                    let arr_element_ptr = unsafe { arr_ptr.add(i as usize * element_size) };
                    element.write_type(arr_element_ptr, out);

                    if i == num_items - 1 {
                        break;
                    }

                    let mut prod = 1;
                    let rank_step = dims
                        .iter()
                        .rev()
                        .take_while(|d| {
                            prod *= **d;
                            (i + 1) % prod == 0
                        })
                        .count();

                    if rank_step > 0 {
                        out.extend(repeat_n(';', rank_step));
                    } else {
                        out.push(',');
                    }
                    out.push_str(" ")
                }
                out.push(']');
            }
        }
    }
}
