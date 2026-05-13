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

pub static BARCODE_VALID_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 -$%./+*";

#[derive(Debug, PartialEq)]
pub struct SegmentPair {
    pub width: &'static str, // Width of barcode segment
    pub x: &'static str,     // X position of segment
}

#[derive(Debug, PartialEq)]
pub struct Code39Segment {
    pub v_lines: [SegmentPair; 5], // Vertical barcode lines
}

pub static CODE_39_BARCODE_SEGMENTS: LazyLock<HashMap<char, Code39Segment>> = LazyLock::new(|| {
    HashMap::from([
        (
            'A',
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
            'B',
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
            'C',
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
            'D',
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
            'E',
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
            'F',
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
            'G',
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
            'H',
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
            'I',
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
            'J',
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
            'K',
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
            'L',
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
            'M',
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
            'N',
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
            'O',
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
            'P',
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
            'Q',
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
                        x: X5,
                    },
                    SegmentPair {
                        width: WIDTH_LARGE,
                        x: XB,
                    },
                ],
            },
        ),
        (
            'R',
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
            'S',
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
            'T',
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
            'U',
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
            'V',
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
            'W',
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
            'X',
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
            'Y',
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
            'Z',
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
        (
            '0',
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
            '1',
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
            '2',
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
            '3',
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
            '4',
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
            '5',
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
            '6',
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
            '7',
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
            '8',
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
            '9',
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
            ' ',
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
            '-',
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
            '$',
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
            '%',
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
            '.',
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
            '/',
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
            '+',
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
            '*',
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
                CODE_39_BARCODE_SEGMENTS.get(&char).is_some(),
                "Missing character"
            );
        }

        // Check for duplicate values
        for (index, char) in BARCODE_VALID_CHARS.chars().enumerate() {
            let segment = CODE_39_BARCODE_SEGMENTS.get(&char).unwrap();
            for check_char in BARCODE_VALID_CHARS.chars().skip(index + 1) {
                let check_segment = CODE_39_BARCODE_SEGMENTS.get(&check_char).unwrap();
                // println!("Check {} {}", char, check_char);
                assert_ne!(segment, check_segment, "Duplicate found");
            }
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
