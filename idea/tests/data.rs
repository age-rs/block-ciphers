#[derive(Copy, Clone)]
pub struct IdeaTest {
    pub key: [u8; 16],
    pub plain_text: [u8; 8],
    pub cipher_text: [u8; 8],
}

// Test vectors from: https://www.cosic.esat.kuleuven.be/nessie/testvectors/bc/idea/Idea-128-64.verified.test-vectors

// Set 1, vector#127:
// key=00000000000000000000000000000001
// plain=0000000000000000
// cipher=C57ADBDE27BC26CF

pub const TEST_VECTOR_1: IdeaTest = IdeaTest {
    key: [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
    ],
    plain_text: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    cipher_text: [0xc5, 0x7a, 0xdb, 0xde, 0x27, 0xbc, 0x26, 0xcf],
};

// Set 2, vector# 63:
// key=00000000000000000000000000000000
// plain=0000000000000001
// cipher=0013FFF500120009

pub const TEST_VECTOR_2: IdeaTest = IdeaTest {
    key: [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    plain_text: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
    cipher_text: [0x00, 0x13, 0xff, 0xf5, 0x00, 0x12, 0x00, 0x09],
};

// Set 3, vector#255:
// key=FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
// plain=FFFFFFFFFFFFFFFF
// cipher=CD1AB2C1211041FB

pub const TEST_VECTOR_3: IdeaTest = IdeaTest {
    key: [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff,
    ],
    plain_text: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    cipher_text: [0xcd, 0x1a, 0xb2, 0xc1, 0x21, 0x10, 0x41, 0xfb],
};

// Set 4, vector#  1:
// key=2BD6459F82C5B300952C49104881FF48
// plain=EA024714AD5C4D84
// cipher=C8FB51D3516627A8

pub const TEST_VECTOR_4: IdeaTest = IdeaTest {
    key: [
        0x2b, 0xd6, 0x45, 0x9f, 0x82, 0xc5, 0xb3, 0x00, 0x95, 0x2c, 0x49, 0x10,
        0x48, 0x81, 0xff, 0x48,
    ],
    plain_text: [0xea, 0x02, 0x47, 0x14, 0xad, 0x5c, 0x4d, 0x84],
    cipher_text: [0xc8, 0xfb, 0x51, 0xd3, 0x51, 0x66, 0x27, 0xa8],
};

// Set 5, vector#127:
// key=00000000000000000000000000000001
// plain=F61D37EC1C099DE5
// cipher=0000000000000000

pub const TEST_VECTOR_5: IdeaTest = IdeaTest {
    key: [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
    ],
    plain_text: [0xf6, 0x1d, 0x37, 0xec, 0x1c, 0x09, 0x9d, 0xe5],
    cipher_text: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

// Set 6, vector# 63:
// key=00000000000000000000000000000000
// plain=0013FFF500120009
// cipher=0000000000000001

pub const TEST_VECTOR_6: IdeaTest = IdeaTest {
    key: [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    plain_text: [0x00, 0x13, 0xff, 0xf5, 0x00, 0x12, 0x00, 0x09],
    cipher_text: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
};

// Set 7, vector#255:
// key=FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
// plain=28886D814399E782
// cipher=FFFFFFFFFFFFFFFF

pub const TEST_VECTOR_7: IdeaTest = IdeaTest {
    key: [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff,
    ],
    plain_text: [0x28, 0x88, 0x6d, 0x81, 0x43, 0x99, 0xe7, 0x82],
    cipher_text: [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
};

// Set 8, vector#  1:
// key=2BD6459F82C5B300952C49104881FF48
// plain=F129A6601EF62A47
// cipher=EA024714AD5C4D84

pub const TEST_VECTOR_8: IdeaTest = IdeaTest {
    key: [
        0x2b, 0xd6, 0x45, 0x9f, 0x82, 0xc5, 0xb3, 0x00, 0x95, 0x2c, 0x49, 0x10,
        0x48, 0x81, 0xff, 0x48,
    ],
    plain_text: [0xf1, 0x29, 0xa6, 0x60, 0x1e, 0xf6, 0x2a, 0x47],
    cipher_text: [0xea, 0x02, 0x47, 0x14, 0xad, 0x5c, 0x4d, 0x84],
};