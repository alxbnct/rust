/*
Module: f32

Floating point operations and constants for `f32`
*/
// PORT

import cmath::c_float::*;

type t = f32;

/* Const: NaN */
const NaN: f32 = 0.0f32/0.0f32;

/* Const: infinity */
const infinity: f32 = 1.0f32/0.0f32;

/* Const: neg_infinity */
const neg_infinity: f32 = -1.0f32/0.0f32;

/* Predicate: isNaN */
pure fn isNaN(f: f32) -> bool { f != f }

/* Function: add */
pure fn add(x: f32, y: f32) -> f32 { ret x + y; }

/* Function: sub */
pure fn sub(x: f32, y: f32) -> f32 { ret x - y; }

/* Function: mul */
pure fn mul(x: f32, y: f32) -> f32 { ret x * y; }

/* Function: div */
pure fn div(x: f32, y: f32) -> f32 { ret x / y; }

/* Function: rem */
pure fn rem(x: f32, y: f32) -> f32 { ret x % y; }

/* Predicate: lt */
pure fn lt(x: f32, y: f32) -> bool { ret x < y; }

/* Predicate: le */
pure fn le(x: f32, y: f32) -> bool { ret x <= y; }

/* Predicate: eq */
pure fn eq(x: f32, y: f32) -> bool { ret x == y; }

/* Predicate: ne */
pure fn ne(x: f32, y: f32) -> bool { ret x != y; }

/* Predicate: ge */
pure fn ge(x: f32, y: f32) -> bool { ret x >= y; }

/* Predicate: gt */
pure fn gt(x: f32, y: f32) -> bool { ret x > y; }

/*
Predicate: positive

Returns true if `x` is a positive number, including +0.0f320 and +Infinity.
 */
pure fn positive(x: f32) -> bool
    { ret x > 0.0f32 || (1.0f32/x) == infinity; }

/*
Predicate: negative

Returns true if `x` is a negative number, including -0.0f320 and -Infinity.
 */
pure fn negative(x: f32) -> bool
    { ret x < 0.0f32 || (1.0f32/x) == neg_infinity; }

/*
Predicate: nonpositive

Returns true if `x` is a negative number, including -0.0f320 and -Infinity.
(This is the same as `f32::negative`.)
*/
pure fn nonpositive(x: f32) -> bool {
  ret x < 0.0f32 || (1.0f32/x) == neg_infinity;
}

/*
Predicate: nonnegative

Returns true if `x` is a positive number, including +0.0f320 and +Infinity.
(This is the same as `f32::positive`.)
*/
pure fn nonnegative(x: f32) -> bool {
  ret x > 0.0f32 || (1.0f32/x) == infinity;
}

/* Module: consts */
mod consts {

    /*
    Const: pi

    Archimedes' constant
    */
    const pi: f32 = 3.14159265358979323846264338327950288f32;

    /*
    Const: frac_pi_2

    pi/2.0
    */
    const frac_pi_2: f32 = 1.57079632679489661923132169163975144f32;

    /*
    Const: frac_pi_4

    pi/4.0
    */
    const frac_pi_4: f32 = 0.785398163397448309615660845819875721f32;

    /*
    Const: frac_1_pi

    1.0/pi
    */
    const frac_1_pi: f32 = 0.318309886183790671537767526745028724f32;

    /*
    Const: frac_2_pi

    2.0/pi
    */
    const frac_2_pi: f32 = 0.636619772367581343075535053490057448f32;

    /*
    Const: frac_2_sqrtpi

    2.0/sqrt(pi)
    */
    const frac_2_sqrtpi: f32 = 1.12837916709551257389615890312154517f32;

    /*
    Const: sqrt2

    sqrt(2.0)
    */
    const sqrt2: f32 = 1.41421356237309504880168872420969808f32;

    /*
    Const: frac_1_sqrt2

    1.0/sqrt(2.0)
    */
    const frac_1_sqrt2: f32 = 0.707106781186547524400844362104849039f32;

    /*
    Const: e

    Euler's number
    */
    const e: f32 = 2.71828182845904523536028747135266250f32;

    /*
    Const: log2_e

    log2(e)
    */
    const log2_e: f32 = 1.44269504088896340735992468100189214f32;

    /*
    Const: log10_e

    log10(e)
    */
    const log10_e: f32 = 0.434294481903251827651128918916605082f32;

    /*
    Const: ln_2

    ln(2.0)
    */
    const ln_2: f32 = 0.693147180559945309417232121458176568f32;

    /*
    Const: ln_10

    ln(10.0)
    */
    const ln_10: f32 = 2.30258509299404568401799145468436421f32;
}

// These are not defined inside consts:: for consistency with
// the integer types

// PORT check per architecture

const radix: uint = 2u;

const mantissa_digits: uint = 24u;
const digits: uint = 6u;

const epsilon: f32 = 1.19209290e-07f32;

const min_value: f32 = 1.17549435e-38f32;
const max_value: f32 = 3.40282347e+38f32;

const min_exp: int = -125;
const max_exp: int = 128;

const min_10_exp: int = -37;
const max_10_exp: int = 38;

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
