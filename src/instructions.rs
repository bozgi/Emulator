trait Instructions {
    fn mov(&mut self, src: i32, dst: i32);
    fn set(&mut self, src: i32, dst: i32);
    fn ld(src: i32, dst: i32);
    fn st(src: i32, dst: i32);
    fn add(src: i32, dst: i32);
    fn sub(src: i32, dst: i32);
    fn mul(src: i32, dst: i32);
    fn div(src: i32, dst: i32);
    fn or(src: i32, dst: i32);
    fn and(src: i32, dst: i32);
    fn xor(src: i32, dst: i32);
    fn not(src: i32);
    fn shr(src: i32, dst: i32);
    fn shl(src: i32, dst: i32);
    fn cmp(src: i32, dst: i32);
    fn jmp(src: i32, dst: i32);
    fn jnz(src: i32, dst: i32);
    fn jnn(src: i32, dst: i32);
}