#![cfg(feature = "with-decimal")]

use core::{marker::PhantomData, ptr};

use rust_decimal::{prelude::ToPrimitive, Decimal, MathematicalOps};

use crate::{
    constants::TABLE, sealed::Sealed, to_formatted_str::ToFormattedStr, Buffer, Format, Grouping,
};

impl Sealed for Decimal {}

impl ToFormattedStr for Decimal {
    fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, format: &F) -> usize
    where
        F: Format,
    {
        let integral = self.round().normalize();
        let scale = self.scale();
        let decimal = ((self - integral) * Decimal::TEN.powu(scale as u64))
            .round()
            .abs()
            .normalize();
        let integral = self
            .round()
            .normalize()
            .to_i128()
            .expect("Improper integral.");
        let mut aux_size: usize = 0;

        if scale != 0 {
            let decimal = format!("{:0<1$}", decimal, scale as usize);
            let decimal = decimal.as_str();
            let decimal_len = decimal.len();

            buf.pos -= decimal_len;

            for (i, byte) in decimal.as_bytes().iter().enumerate() {
                buf.inner[buf.pos + i] = *byte;
            }

            aux_size += decimal_len;

            let decimal_point = format.decimal().into_str();
            let decimal_point_len = decimal_point.len();

            buf.pos -= decimal_point_len;

            for (i, byte) in decimal_point.as_bytes().iter().enumerate() {
                buf.inner[buf.pos + i] = *byte;
            }

            aux_size += decimal_point_len;
        }

        let c = if self.is_sign_negative() {
            let n = (!(integral as u128)).wrapping_add(1); // make positive by adding 1 to the 2s complement
            let c = run_core_algorithm(n, buf, format);
            let minus_sign = format.minus_sign().into_str();
            let min_len = minus_sign.len();
            buf.pos -= min_len;
            for (i, byte) in minus_sign.as_bytes().iter().enumerate() {
                buf.inner[buf.pos + i] = *byte;
            }

            aux_size += min_len;

            c
        } else {
            run_core_algorithm(integral as u128, buf, format)
        };

        c + aux_size
    }
}

fn run_core_algorithm<F>(mut n: u128, buf: &mut Buffer, format: &F) -> usize
where
    F: Format,
{
    // Bail out early if we can just use itoa
    // (i.e. if we don't have a separator)
    let separator = format.separator().into_str();
    let grouping = format.grouping();
    if separator.is_empty() || grouping == Grouping::Posix {
        let mut itoa_buf = itoa::Buffer::new();

        let s = itoa_buf.format(n);
        let s_len = s.len();
        let end = buf.pos;

        buf.pos -= s_len;

        let dst = &mut buf.inner[buf.pos..end];
        dst.copy_from_slice(s.as_bytes());

        return s_len;
    }

    // Collect separator information
    let mut sep = Sep {
        ptr: separator.as_bytes().as_ptr(),
        len: separator.len(),
        pos: buf.pos as isize - 4,
        step: match grouping {
            Grouping::Standard => 4isize,
            Grouping::Indian => 3isize,
            Grouping::Posix => unreachable!(),
        },
        phantom: PhantomData,
    };

    // Start the main algorithm
    while n >= 10_000 {
        let remainder = n % 10_000;
        let table_index = ((remainder % 100) << 1) as isize;
        write_two_bytes(buf, &mut sep, table_index);
        let table_index = ((remainder / 100) << 1) as isize;
        write_two_bytes(buf, &mut sep, table_index);
        n /= 10_000;
    }
    let mut n = n as isize;
    while n >= 100 {
        let table_index = (n % 100) << 1;
        write_two_bytes(buf, &mut sep, table_index);
        n /= 100;
    }
    if n >= 10 {
        let table_index = n << 1;
        write_two_bytes(buf, &mut sep, table_index);
    } else {
        let table_index = n << 1;
        write_one_byte(buf, &mut sep, table_index + 1);
    }

    buf.end - buf.pos
}

struct Sep<'a> {
    ptr: *const u8,
    len: usize,
    pos: isize,
    step: isize,
    phantom: PhantomData<&'a ()>,
}

fn write_one_byte(buf: &mut Buffer, sep: &mut Sep<'_>, table_index: isize) {
    buf.pos -= 1;
    if sep.pos == (buf.pos as isize) {
        buf.pos -= sep.len - 1;
        unsafe { ptr::copy_nonoverlapping(sep.ptr, buf.as_mut_ptr().add(buf.pos), sep.len) }
        sep.pos -= sep.step + (sep.len as isize - 1);
        buf.pos -= 1;
    }
    unsafe {
        ptr::copy_nonoverlapping(
            TABLE.as_ptr().offset(table_index),
            buf.as_mut_ptr().add(buf.pos),
            1,
        )
    };
}

#[inline(always)]
fn write_two_bytes(buf: &mut Buffer, sep: &mut Sep<'_>, table_index: isize) {
    write_one_byte(buf, sep, table_index + 1);
    write_one_byte(buf, sep, table_index);
}
