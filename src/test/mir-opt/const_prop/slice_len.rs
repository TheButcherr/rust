// EMIT_MIR rustc.main.ConstProp.diff

fn main() {
    (&[1u32, 2, 3] as &[u32])[1];
}
