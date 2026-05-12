use std::{collections::HashMap, sync::LazyLock};

static WIDTH_SMALL: &str = "0.432"; // Small barcode line
static WIDTH_LARGE: &str = "1.188"; // Large barcode line

static X0: &str = "0.216"; // X positions
static X1: &str = "1.080";
static X2: &str = "1.836";
static X3: &str = "1.944";
static X4: &str = "2.592";
static X5: &str = "2.700";
static X6: &str = "3.456";
static X7: &str = "3.564";
static X8: &str = "4.212";
static X9: &str = "4.320";
static XA: &str = "5.076";
static XB: &str = "5.184";
static XC: &str = "5.940";

pub static BARCODE_VALID_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ* -$%./+";

pub struct SegmentPair {
    pub width: &'static str, // Width of barcode segment
    pub x: &'static str,     // X position of segment
}

pub struct Code39Segment {
    pub v_lines: [SegmentPair; 5], // Vertical barcode lines
}

pub static CODE_39_BARCODE_SEGMENTS: LazyLock<HashMap<i64, Code39Segment>> = LazyLock::new(|| {
    HashMap::from([
        (
            32, // Space character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            36, // $ character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            37, // % character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            42, // * character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            43, // + character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            45, // - character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            46, // . character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X4,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            47, // / character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            48, // 0 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            49, // 1 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            50, // 2 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            51, // 3 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X8,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            52, // 4 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            53, // 5 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            54, // 6 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            55, // 7 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            56, // 8 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            57, // 9 character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            65, // A character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            66, // B character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            67, // C character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            68, // D character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X3,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            69, // E character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            70, // F character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            71, // G character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X3,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            72, // H character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            73, // I character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            74, // J character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X3,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            75, // K character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            76, // L character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            77, // M character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            78, // N character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X3,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            79, // O character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            80, // P character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            81, // Q character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X3,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            82, // R character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            83, // S character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            84, // T character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X1,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X3,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X7,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            85, // U character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X4,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            86, // V character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            87, // W character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X4,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X8,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            88, // X character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X9,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            89, // Y character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X4,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
        (
            90, // Z character
            Code39Segment {
                v_lines: [
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: X0,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X2,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: X6,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XA,
                    },
                    SegmentPair {
                        width: WIDTH_SMALL,
                        x: XC,
                    },
                ],
            },
        ),
    ])
});

impl Code39Segment {
    pub fn create_segment(&self, height: &str) -> [String; 5] {
        let mut result_array: [String; 5] = Default::default();

        for (result_element, sp) in result_array.iter_mut().zip(&self.v_lines) {
            *result_element = format!(
                "<rect width=\"{}\" height=\"{}\" x=\"{}\" y=\"0\" />",
                sp.width, height, sp.x
            )
        }
        result_array
    }
}

#[cfg(test)]
mod tests {
    use super::{BARCODE_VALID_CHARS, CODE_39_BARCODE_SEGMENTS};

    #[test]
    fn check() {
        for char in BARCODE_VALID_CHARS.chars() {
            assert!(
                CODE_39_BARCODE_SEGMENTS.get(&(char as i64)).is_some(),
                "Missing character"
            );
        }

        assert_eq!(
            BARCODE_VALID_CHARS.len(),
            44,
            "Code 39 barcode supports 44 characters" // Check do not have any extras
        );

        assert_eq!(
            CODE_39_BARCODE_SEGMENTS.len(),
            44,
            "Code 39 barcode supports 44 characters" // Check do not have any extras
        );
    }
}
