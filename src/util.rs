use std::convert::AsMut;

const W_SIZE: usize = 8;
const X_SIZE: usize = 4;
const Y_SIZE: usize = 2;
const Z_SIZE: usize = 1;
pub const TOTAL_SIZE: usize = W_SIZE + X_SIZE + Y_SIZE + Z_SIZE;

const W_TERM: usize = W_SIZE;
const X_TERM: usize = W_TERM + X_SIZE;
const Y_TERM: usize = X_TERM + Y_SIZE;
const Z_TERM: usize = Y_TERM + Z_SIZE;
    
pub struct Myproto {
    pub w: [u8; W_SIZE],
    pub x: [u8; X_SIZE],
    pub y: [u8; Y_SIZE],
    pub z: [u8; Z_SIZE],
}

pub fn deserialize(buf: &[u8]) -> Myproto {
    // TODO: check buf.len to see if we can deserialize
    Myproto {
        w: clone_into_array(&buf[0..W_TERM]),
        x: clone_into_array(&buf[W_TERM..X_TERM]),
        y: clone_into_array(&buf[X_TERM..Y_TERM]),
        z: clone_into_array(&buf[Y_TERM..Z_TERM]),
    }
}

pub fn serialize(proto: Myproto) -> [u8; TOTAL_SIZE] {
    let mut buffer: [u8; TOTAL_SIZE] = [0; TOTAL_SIZE];
    let mut offset = 0;
    for i in 0..proto.w.len() {
        buffer[offset] = proto.w[i];
        offset += 1;
    }

    for i in 0..proto.x.len() {
        buffer[offset] = proto.x[i];
        offset += 1;
    }

    for i in 0..proto.y.len() {
        buffer[offset] = proto.y[i];
        offset += 1;
    }

    for i in 0..proto.z.len() {
        buffer[offset] = proto.z[i];
        offset += 1;
    }
    buffer
}

pub fn display(heading: &str, proto: Myproto) {
    println!("{} MyProto", heading);
    print!("w: ");
    for i in proto.w.iter() {
        print!("{} ",i);
    }
    println!("");

    print!("x: ");
    for i in proto.x.iter() {
        print!("{} ",i);
    }
    println!("");

    print!("y: ");
    for i in proto.y.iter() {
        print!("{} ",i);
    }
    println!("");

    print!("z: ");
    for i in proto.z.iter() {
        print!("{} ",i);
    }
    println!("");
}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
