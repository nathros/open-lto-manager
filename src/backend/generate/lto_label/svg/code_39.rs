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
                "<rect width=\"{}\" height=\"{}\" x=\"{}\" />",
                sp.width, height, sp.x
            )
        }
        result_array
    }

    fn accumulate_checksum(str: &str) -> u32 {
        let mut sum = 0;
        for c in str.chars() {
            match c {
                '0'..='9' => sum += c as u32 - '0' as u32,
                'A'..='Z' => sum += c as u32 - 'A' as u32 + 10,
                '-' => sum += 36,
                '.' => sum += 37,
                ' ' => sum += 38,
                '$' => sum += 39,
                '/' => sum += 40,
                '+' => sum += 41,
                '%' => sum += 42,
                _ => {}
            }
        }
        sum
    }

    pub fn create_check_digit_mod_10(str: &str) -> String {
        let mut sum = Self::accumulate_checksum(str);

        sum %= 10;
        sum.to_string()
    }

    pub fn create_check_digit_mod_43(str: &str) -> String {
        let mut sum = Self::accumulate_checksum(str);

        sum %= 43;

        match sum {
            0..=9 => sum.to_string(),
            10..=35 => char::from_u32('A' as u32 + sum - 10)
                .unwrap_or('?')
                .to_string(),
            36 => '-'.to_string(),
            37 => '.'.to_string(),
            38 => ' '.to_string(),
            39 => '$'.to_string(),
            40 => '/'.to_string(),
            41 => '+'.to_string(),
            42 => '%'.to_string(),
            _ => '?'.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        backend::generate::lto_label::svg::code_39::Code39Segment, shared::r#const::Const,
    };

    use super::CODE_39_BARCODE_SEGMENTS;

    #[test]
    fn check_segments() {
        for char in Const::BARCODE_VALID_CHARS.chars() {
            assert!(
                CODE_39_BARCODE_SEGMENTS.get(&char).is_some(),
                "Missing character"
            );
        }

        // Check for duplicate values
        for (index, char) in Const::BARCODE_VALID_CHARS.chars().enumerate() {
            let segment = CODE_39_BARCODE_SEGMENTS.get(&char).unwrap();
            for check_char in Const::BARCODE_VALID_CHARS.chars().skip(index + 1) {
                let check_segment = CODE_39_BARCODE_SEGMENTS.get(&check_char).unwrap();
                // println!("Check {} {}", char, check_char);
                assert_ne!(segment, check_segment, "Duplicate found");
            }
        }

        assert_eq!(
            Const::BARCODE_VALID_CHARS.len(),
            44,
            "Code 39 barcode supports 44 characters" // Check do not have any extras
        );

        assert_eq!(
            CODE_39_BARCODE_SEGMENTS.len(),
            44,
            "Code 39 barcode supports 44 characters" // Check do not have any extras
        );
    }

    #[test]
    fn create_check_digits_mod_10() {
        let test_data = [
            ("ABC123", "9"),
            ("ABC124", "0"),
            ("ABC125", "1"),
            ("ABC126", "2"),
            ("ABC127", "3"),
            ("ABC128", "4"),
            ("ABC129", "5"),
            ("ABC139", "6"),
            ("ABD139", "7"),
            ("ABE139", "8"),
            ("BBE139", "9"),
            ("P0001SL5", "0"),
        ];
        for (barcode, check) in test_data {
            assert_eq!(
                Code39Segment::create_check_digit_mod_10(barcode),
                check.to_string()
            );
        }
    }

    #[test]
    fn create_check_digits_mod_43() {
        let test_data = [
            ("DEF456", "E"),
            ("P0001SL5", "."),
            ("P0002SL5", " "),
            ("P0003SL5", "$"),
            ("P0004SL5", "/"),
            ("P0005SL5", "+"),
            ("P0006SL5", "%"),
            ("P0007SL5", "0"),
            ("P0008SL5", "1"),
            ("P0088SL5", "9"),
            ("P0089SL5", "A"),
            ("P0099SL5", "B"),
            ("P0999SL5", "K"),
            ("P9999SL5", "T"),
            ("P9999YL5", "Z"),
            ("P9999YL6", "-"),
            ("P9999YL7", "."),
        ];
        for (barcode, check) in test_data {
            assert_eq!(
                Code39Segment::create_check_digit_mod_43(barcode),
                check.to_string()
            );
        }
    }
}
