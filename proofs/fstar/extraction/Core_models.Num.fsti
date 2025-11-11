module Core_models.Num
open Rust_primitives
 
val trailing_zeros: #t:inttype -> int_t t -> (n:u32{v n >= 0 /\ v n <= bits t})

unfold let impl_u8__trailing_zeros (n:u8) = trailing_zeros n
unfold let impl_u16__trailing_zeros (n:u16) = trailing_zeros n
unfold let impl_u32__trailing_zeros (n:u32) = trailing_zeros n
unfold let impl_u64__trailing_zeros (n:u64) = trailing_zeros n
unfold let impl_u128__trailing_zeros (n:u128) = trailing_zeros n
unfold let impl_usize__trailing_zeros (n:usize) = trailing_zeros n


val trailing_zeros_lt_bits #t (a: int_t t):
    Lemma (requires (v a <> 0))
          (ensures (v (trailing_zeros a) < bits t))
          [SMTPat (trailing_zeros a)]

val trailing_zeros_band_le_left #t (a b : int_t t):
    Lemma (v (trailing_zeros (a |. b)) <= v (trailing_zeros a))
          [SMTPat (trailing_zeros (a |. b))]

val trailing_zeros_band_le_right #t (a b : int_t t):
    Lemma (v (trailing_zeros (a |. b)) <= v (trailing_zeros b))
          [SMTPat (trailing_zeros (a |. b))]

val shift_right_trailing_zeros_nonzero #t (a: int_t t) (b : u32):
    Lemma (requires (v a <> 0) && (v b <= v (trailing_zeros a)))
          (ensures (v (shift_right a b) <> 0))
          [SMTPat (shift_right a b)]

val shift_right_trailing_zeros_le #t (a: int_t t):
    Lemma (requires (v a <> 0))
          (ensures (v (shift_right a (trailing_zeros a)) <= v a))
          [SMTPat (shift_right a (trailing_zeros a))]

val modulo_lemma #t (a b x: int_t t) :
    Lemma (requires (v x <> 0 && v a <> 0 && v (a %! x) = 0))
          (ensures ((b %! a) %! x == b %! x))
          [SMTPat ((b %! a) %! x)]