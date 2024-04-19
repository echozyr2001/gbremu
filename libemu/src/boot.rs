pub enum BootRom {
  Dmg,
  Sgb,
  DmgBootix,
  MgbBootix,
  Cgb,
  None,
}

pub const DMG_BOOT: [u8; 256] = [
  49, 254, 255, 175, 33, 255, 159, 50, 203, 124, 32, 251, 33, 38, 255, 14, 17, 62, 128, 50, 226,
  12, 62, 243, 226, 50, 62, 119, 119, 62, 252, 224, 71, 17, 4, 1, 33, 16, 128, 26, 205, 149, 0,
  205, 150, 0, 19, 123, 254, 52, 32, 243, 17, 216, 0, 6, 8, 26, 19, 34, 35, 5, 32, 249, 62, 25,
  234, 16, 153, 33, 47, 153, 14, 12, 61, 40, 8, 50, 13, 32, 249, 46, 15, 24, 243, 103, 62, 100, 87,
  224, 66, 62, 145, 224, 64, 4, 30, 2, 14, 12, 240, 68, 254, 144, 32, 250, 13, 32, 247, 29, 32,
  242, 14, 19, 36, 124, 30, 131, 254, 98, 40, 6, 30, 193, 254, 100, 32, 6, 123, 226, 12, 62, 135,
  226, 240, 66, 144, 224, 66, 21, 32, 210, 5, 32, 79, 22, 32, 24, 203, 79, 6, 4, 197, 203, 17, 23,
  193, 203, 17, 23, 5, 32, 245, 34, 35, 34, 35, 201, 206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0,
  131, 0, 12, 0, 13, 0, 8, 17, 31, 136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187,
  187, 103, 99, 110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62, 60, 66, 185, 165, 185,
  165, 66, 60, 33, 4, 1, 17, 168, 0, 26, 19, 190, 32, 254, 35, 125, 254, 52, 32, 245, 6, 25, 120,
  134, 35, 5, 32, 251, 134, 32, 254, 62, 1, 224, 80,
];

pub const SGB_BOOT: [u8; 256] = [
  49, 254, 255, 62, 48, 224, 0, 175, 33, 255, 159, 50, 203, 124, 32, 251, 33, 38, 255, 14, 17, 62,
  128, 50, 226, 12, 62, 243, 226, 50, 62, 119, 119, 62, 252, 224, 71, 33, 95, 192, 14, 8, 175, 50,
  13, 32, 252, 17, 79, 1, 62, 251, 14, 6, 245, 6, 0, 26, 27, 50, 128, 71, 13, 32, 248, 50, 241, 50,
  14, 14, 214, 2, 254, 239, 32, 234, 17, 4, 1, 33, 16, 128, 26, 205, 211, 0, 205, 212, 0, 19, 123,
  254, 52, 32, 243, 17, 230, 0, 6, 8, 26, 19, 34, 35, 5, 32, 249, 62, 25, 234, 16, 153, 33, 47,
  153, 14, 12, 61, 40, 8, 50, 13, 32, 249, 46, 15, 24, 243, 62, 145, 224, 64, 33, 0, 192, 14, 0,
  62, 0, 226, 62, 48, 226, 6, 16, 30, 8, 42, 87, 203, 66, 62, 16, 32, 2, 62, 32, 226, 62, 48, 226,
  203, 26, 29, 32, 239, 5, 32, 232, 62, 32, 226, 62, 48, 226, 205, 194, 0, 125, 254, 96, 32, 210,
  14, 19, 62, 193, 226, 12, 62, 7, 226, 24, 58, 22, 4, 240, 68, 254, 144, 32, 250, 30, 0, 29, 32,
  253, 21, 32, 242, 201, 79, 6, 4, 197, 203, 17, 23, 193, 203, 17, 23, 5, 32, 245, 34, 35, 34, 35,
  201, 60, 66, 185, 165, 185, 165, 66, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 1, 224,
  80,
];

pub const DMG_BOOTIX: [u8; 256] = [
  49, 254, 255, 33, 255, 159, 175, 50, 203, 124, 32, 250, 14, 17, 33, 38, 255, 62, 128, 50, 226,
  12, 62, 243, 50, 226, 12, 62, 119, 50, 226, 17, 4, 1, 33, 16, 128, 26, 205, 184, 0, 26, 203, 55,
  205, 184, 0, 19, 123, 254, 52, 32, 240, 17, 204, 0, 6, 8, 26, 19, 34, 35, 5, 32, 249, 33, 4, 153,
  1, 12, 1, 205, 177, 0, 62, 25, 119, 33, 36, 153, 14, 12, 205, 177, 0, 62, 145, 224, 64, 6, 16,
  17, 212, 0, 120, 224, 67, 5, 123, 254, 216, 40, 4, 26, 224, 71, 19, 14, 28, 205, 167, 0, 175,
  144, 224, 67, 5, 14, 28, 205, 167, 0, 175, 176, 32, 224, 224, 67, 62, 131, 205, 159, 0, 14, 39,
  205, 167, 0, 62, 193, 205, 159, 0, 17, 138, 1, 240, 68, 254, 144, 32, 250, 27, 122, 179, 32, 245,
  24, 73, 14, 19, 226, 12, 62, 135, 226, 201, 240, 68, 254, 144, 32, 250, 13, 32, 247, 201, 120,
  34, 4, 13, 32, 250, 201, 71, 14, 4, 175, 197, 203, 16, 23, 193, 203, 16, 23, 13, 32, 245, 34, 35,
  34, 35, 201, 60, 66, 185, 165, 185, 165, 66, 60, 0, 84, 168, 252, 66, 79, 79, 84, 73, 88, 46, 68,
  77, 71, 32, 118, 49, 46, 50, 0, 62, 255, 198, 1, 11, 30, 216, 33, 77, 1, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 62, 1, 224, 80,
];

pub const MGB_BOOTIX: [u8; 256] = [
  49, 254, 255, 33, 255, 159, 175, 50, 203, 124, 32, 250, 14, 17, 33, 38, 255, 62, 128, 50, 226,
  12, 62, 243, 50, 226, 12, 62, 119, 50, 226, 17, 4, 1, 33, 16, 128, 26, 205, 184, 0, 26, 203, 55,
  205, 184, 0, 19, 123, 254, 52, 32, 240, 17, 204, 0, 6, 8, 26, 19, 34, 35, 5, 32, 249, 33, 4, 153,
  1, 12, 1, 205, 177, 0, 62, 25, 119, 33, 36, 153, 14, 12, 205, 177, 0, 62, 145, 224, 64, 6, 16,
  17, 212, 0, 120, 224, 67, 5, 123, 254, 216, 40, 4, 26, 224, 71, 19, 14, 30, 205, 167, 0, 175,
  144, 224, 67, 5, 14, 30, 205, 167, 0, 175, 176, 32, 224, 224, 67, 62, 131, 205, 159, 0, 14, 30,
  205, 167, 0, 62, 193, 205, 159, 0, 17, 242, 1, 240, 68, 254, 144, 32, 250, 27, 122, 179, 32, 245,
  24, 73, 14, 19, 226, 12, 62, 135, 226, 201, 240, 68, 254, 144, 32, 250, 13, 32, 247, 201, 120,
  34, 4, 13, 32, 250, 201, 71, 14, 4, 175, 197, 203, 16, 23, 193, 203, 16, 23, 13, 32, 245, 34, 35,
  34, 35, 201, 60, 66, 185, 165, 185, 165, 66, 60, 0, 84, 168, 252, 66, 79, 79, 84, 73, 88, 46, 68,
  77, 71, 32, 118, 49, 46, 50, 0, 62, 255, 198, 1, 11, 30, 216, 33, 77, 1, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 62, 255, 224, 80,
];

pub const CGB_BOOT: [u8; 2304] = [
  49, 254, 255, 62, 2, 195, 124, 0, 211, 0, 152, 160, 18, 211, 0, 128, 0, 64, 30, 83, 208, 0, 31,
  66, 28, 0, 20, 42, 77, 25, 140, 126, 0, 124, 49, 110, 74, 69, 82, 74, 0, 0, 255, 83, 31, 124,
  255, 3, 31, 0, 255, 31, 167, 0, 239, 27, 31, 0, 239, 27, 0, 124, 0, 0, 255, 3, 206, 237, 102,
  102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0, 8, 17, 31, 136, 137, 0, 14, 220, 204, 110,
  230, 221, 221, 217, 153, 187, 187, 103, 99, 110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51,
  62, 60, 66, 185, 165, 185, 165, 66, 60, 88, 67, 224, 112, 62, 252, 224, 71, 205, 117, 2, 205, 0,
  2, 38, 208, 205, 3, 2, 33, 0, 254, 14, 160, 175, 34, 13, 32, 252, 17, 4, 1, 33, 16, 128, 76, 26,
  226, 12, 205, 198, 3, 205, 199, 3, 19, 123, 254, 52, 32, 241, 17, 114, 0, 6, 8, 26, 19, 34, 35,
  5, 32, 249, 205, 240, 3, 62, 1, 224, 79, 62, 145, 224, 64, 33, 178, 152, 6, 78, 14, 68, 205, 145,
  2, 175, 224, 79, 14, 128, 33, 66, 0, 6, 24, 242, 12, 190, 32, 254, 35, 5, 32, 247, 33, 52, 1, 6,
  25, 120, 134, 44, 5, 32, 251, 134, 32, 254, 205, 28, 3, 24, 2, 0, 0, 205, 208, 5, 175, 224, 112,
  62, 17, 224, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 33, 0, 128, 175, 34, 203, 108, 40, 251, 201, 42, 18, 19, 13, 32, 250, 201, 229,
  33, 15, 255, 203, 134, 203, 70, 40, 252, 225, 201, 17, 0, 255, 33, 3, 208, 14, 15, 62, 48, 18,
  62, 32, 18, 26, 47, 161, 203, 55, 71, 62, 16, 18, 26, 47, 161, 176, 79, 126, 169, 230, 240, 71,
  42, 169, 161, 176, 50, 71, 121, 119, 62, 48, 18, 201, 62, 128, 224, 104, 224, 106, 14, 107, 42,
  226, 5, 32, 251, 74, 9, 67, 14, 105, 42, 226, 5, 32, 251, 201, 197, 213, 229, 33, 0, 216, 6, 1,
  22, 63, 30, 64, 205, 74, 2, 225, 209, 193, 201, 62, 128, 224, 38, 224, 17, 62, 243, 224, 18, 224,
  37, 62, 119, 224, 36, 33, 48, 255, 175, 14, 16, 34, 47, 13, 32, 251, 201, 205, 17, 2, 205, 98, 2,
  121, 254, 56, 32, 20, 229, 175, 224, 79, 33, 167, 153, 62, 56, 34, 60, 254, 63, 32, 250, 62, 1,
  224, 79, 225, 197, 229, 33, 67, 1, 203, 126, 204, 137, 5, 225, 193, 205, 17, 2, 121, 214, 48,
  210, 6, 3, 121, 254, 1, 202, 6, 3, 125, 254, 209, 40, 33, 197, 6, 3, 14, 1, 22, 3, 126, 230, 248,
  177, 34, 21, 32, 248, 12, 121, 254, 6, 32, 240, 17, 17, 0, 25, 5, 32, 231, 17, 161, 255, 25, 193,
  4, 120, 30, 131, 254, 98, 40, 6, 30, 193, 254, 100, 32, 7, 123, 224, 19, 62, 135, 224, 20, 250,
  2, 208, 254, 0, 40, 10, 61, 234, 2, 208, 121, 254, 1, 202, 145, 2, 13, 194, 145, 2, 201, 14, 38,
  205, 74, 3, 205, 17, 2, 205, 98, 2, 13, 32, 244, 205, 17, 2, 62, 1, 224, 79, 205, 62, 3, 205, 65,
  3, 175, 224, 79, 205, 62, 3, 201, 33, 8, 0, 17, 81, 255, 14, 5, 205, 10, 2, 201, 197, 213, 229,
  33, 64, 216, 14, 32, 126, 230, 31, 254, 31, 40, 1, 60, 87, 42, 7, 7, 7, 230, 7, 71, 58, 7, 7, 7,
  230, 24, 176, 254, 31, 40, 1, 60, 15, 15, 15, 71, 230, 224, 178, 34, 120, 230, 3, 95, 126, 15,
  15, 230, 31, 254, 31, 40, 1, 60, 7, 7, 179, 34, 13, 32, 199, 225, 209, 193, 201, 14, 0, 26, 230,
  240, 203, 73, 40, 2, 203, 55, 71, 35, 126, 176, 34, 26, 230, 15, 203, 73, 32, 2, 203, 55, 71, 35,
  126, 176, 34, 19, 203, 65, 40, 13, 213, 17, 248, 255, 203, 73, 40, 3, 17, 8, 0, 25, 209, 12, 121,
  254, 24, 32, 204, 201, 71, 213, 22, 4, 88, 203, 16, 23, 203, 19, 23, 21, 32, 246, 209, 34, 35,
  34, 35, 201, 62, 25, 234, 16, 153, 33, 47, 153, 14, 12, 61, 40, 8, 50, 13, 32, 249, 46, 15, 24,
  243, 201, 62, 1, 224, 79, 205, 0, 2, 17, 7, 6, 33, 128, 128, 14, 192, 26, 34, 35, 34, 35, 19, 13,
  32, 247, 17, 4, 1, 205, 143, 3, 1, 168, 255, 9, 205, 143, 3, 1, 248, 255, 9, 17, 114, 0, 14, 8,
  35, 26, 34, 19, 13, 32, 249, 33, 194, 152, 6, 8, 62, 8, 14, 16, 34, 13, 32, 252, 17, 16, 0, 25,
  5, 32, 243, 175, 224, 79, 33, 194, 152, 62, 8, 34, 60, 254, 24, 32, 2, 46, 226, 254, 40, 32, 3,
  33, 2, 153, 254, 56, 32, 237, 33, 216, 8, 17, 64, 216, 6, 8, 62, 255, 18, 19, 18, 19, 14, 2, 205,
  10, 2, 62, 0, 18, 19, 18, 19, 19, 19, 5, 32, 234, 205, 98, 2, 33, 75, 1, 126, 254, 51, 32, 11,
  46, 68, 30, 48, 42, 187, 32, 73, 28, 24, 4, 46, 75, 30, 1, 42, 187, 32, 62, 46, 52, 1, 16, 0, 42,
  128, 71, 13, 32, 250, 234, 0, 208, 33, 199, 6, 14, 0, 42, 184, 40, 8, 12, 121, 254, 79, 32, 246,
  24, 31, 121, 214, 65, 56, 28, 33, 22, 7, 22, 0, 95, 25, 250, 55, 1, 87, 126, 186, 40, 13, 17, 14,
  0, 25, 121, 131, 79, 214, 94, 56, 237, 14, 0, 33, 51, 7, 6, 0, 9, 126, 230, 31, 234, 8, 208, 126,
  230, 224, 7, 7, 7, 234, 11, 208, 205, 233, 4, 201, 17, 145, 7, 33, 0, 217, 250, 11, 208, 71, 14,
  30, 203, 64, 32, 2, 19, 19, 26, 34, 32, 2, 27, 27, 203, 72, 32, 2, 19, 19, 26, 34, 19, 19, 32, 2,
  27, 27, 203, 80, 40, 5, 27, 43, 26, 34, 19, 26, 34, 19, 13, 32, 215, 33, 0, 217, 17, 0, 218, 205,
  100, 5, 201, 33, 18, 0, 250, 5, 208, 7, 7, 6, 0, 79, 9, 17, 64, 216, 6, 8, 229, 14, 2, 205, 10,
  2, 19, 19, 19, 19, 19, 19, 225, 5, 32, 240, 17, 66, 216, 14, 2, 205, 10, 2, 17, 74, 216, 14, 2,
  205, 10, 2, 43, 43, 17, 68, 216, 14, 2, 205, 10, 2, 201, 14, 96, 42, 229, 197, 33, 232, 7, 6, 0,
  79, 9, 14, 8, 205, 10, 2, 193, 225, 13, 32, 236, 201, 250, 8, 208, 17, 24, 0, 60, 61, 40, 3, 25,
  32, 250, 201, 205, 29, 2, 120, 230, 255, 40, 15, 33, 228, 8, 6, 0, 42, 185, 40, 8, 4, 120, 254,
  12, 32, 246, 24, 45, 120, 234, 5, 208, 62, 30, 234, 2, 208, 17, 11, 0, 25, 86, 122, 230, 31, 95,
  33, 8, 208, 58, 34, 123, 119, 122, 230, 224, 7, 7, 7, 95, 33, 11, 208, 58, 34, 123, 119, 205,
  233, 4, 205, 40, 5, 201, 205, 17, 2, 250, 67, 1, 203, 127, 40, 4, 224, 76, 24, 40, 62, 4, 224,
  76, 62, 1, 224, 108, 33, 0, 218, 205, 123, 5, 6, 16, 22, 0, 30, 8, 205, 74, 2, 33, 122, 0, 250,
  0, 208, 71, 14, 2, 42, 184, 204, 218, 3, 13, 32, 248, 201, 1, 15, 63, 126, 255, 255, 192, 0, 192,
  240, 241, 3, 124, 252, 254, 254, 3, 7, 7, 15, 224, 224, 240, 240, 30, 62, 126, 254, 15, 15, 31,
  31, 255, 255, 0, 0, 1, 1, 1, 3, 255, 255, 225, 224, 192, 240, 249, 251, 31, 127, 248, 224, 243,
  253, 62, 30, 224, 240, 249, 127, 62, 124, 248, 224, 248, 240, 240, 248, 0, 0, 127, 127, 7, 15,
  159, 191, 158, 31, 255, 255, 15, 30, 62, 60, 241, 251, 127, 127, 254, 222, 223, 159, 31, 63, 62,
  60, 248, 248, 0, 0, 3, 3, 7, 7, 255, 255, 193, 192, 243, 231, 247, 243, 192, 192, 192, 192, 31,
  31, 30, 62, 63, 31, 62, 62, 128, 0, 0, 0, 124, 31, 7, 0, 15, 255, 254, 0, 124, 248, 240, 0, 31,
  15, 15, 0, 124, 248, 248, 0, 63, 62, 28, 0, 15, 15, 15, 0, 124, 255, 255, 0, 0, 248, 248, 0, 7,
  15, 15, 0, 129, 255, 255, 0, 243, 225, 128, 0, 224, 255, 127, 0, 252, 240, 192, 0, 62, 124, 124,
  0, 0, 0, 0, 0, 0, 136, 22, 54, 209, 219, 242, 60, 140, 146, 61, 92, 88, 201, 62, 112, 29, 89,
  105, 25, 53, 168, 20, 170, 117, 149, 153, 52, 111, 21, 255, 151, 75, 144, 23, 16, 57, 247, 246,
  162, 73, 78, 67, 104, 224, 139, 240, 206, 12, 41, 232, 183, 134, 154, 82, 1, 157, 113, 156, 189,
  93, 109, 103, 63, 107, 179, 70, 40, 165, 198, 211, 39, 97, 24, 102, 106, 191, 13, 244, 66, 69,
  70, 65, 65, 82, 66, 69, 75, 69, 75, 32, 82, 45, 85, 82, 65, 82, 32, 73, 78, 65, 73, 76, 73, 67,
  69, 32, 82, 124, 8, 18, 163, 162, 7, 135, 75, 32, 18, 101, 168, 22, 169, 134, 177, 104, 160, 135,
  102, 18, 161, 48, 60, 18, 133, 18, 100, 27, 7, 6, 111, 110, 110, 174, 175, 111, 178, 175, 178,
  168, 171, 111, 175, 134, 174, 162, 162, 18, 175, 19, 18, 161, 110, 175, 175, 173, 6, 76, 110,
  175, 175, 18, 124, 172, 168, 106, 110, 19, 160, 45, 168, 43, 172, 100, 172, 109, 135, 188, 96,
  180, 19, 114, 124, 181, 174, 174, 124, 124, 101, 162, 108, 100, 133, 128, 176, 64, 136, 32, 104,
  222, 0, 112, 222, 32, 120, 32, 32, 56, 32, 176, 144, 32, 176, 160, 224, 176, 192, 152, 182, 72,
  128, 224, 80, 30, 30, 88, 32, 184, 224, 136, 176, 16, 32, 0, 16, 32, 224, 24, 224, 24, 0, 24,
  224, 32, 168, 224, 32, 24, 224, 0, 32, 24, 216, 200, 24, 224, 0, 224, 64, 40, 40, 40, 24, 224,
  96, 32, 24, 224, 0, 0, 8, 224, 24, 48, 208, 208, 208, 32, 224, 232, 255, 127, 191, 50, 208, 0, 0,
  0, 159, 99, 121, 66, 176, 21, 203, 4, 255, 127, 49, 110, 74, 69, 0, 0, 255, 127, 239, 27, 0, 2,
  0, 0, 255, 127, 31, 66, 242, 28, 0, 0, 255, 127, 148, 82, 74, 41, 0, 0, 255, 127, 255, 3, 47, 1,
  0, 0, 255, 127, 239, 3, 214, 1, 0, 0, 255, 127, 181, 66, 200, 61, 0, 0, 116, 126, 255, 3, 128, 1,
  0, 0, 255, 103, 172, 119, 19, 26, 107, 45, 214, 126, 255, 75, 117, 33, 0, 0, 255, 83, 95, 74, 82,
  126, 0, 0, 255, 79, 210, 126, 76, 58, 224, 28, 237, 3, 255, 127, 95, 37, 0, 0, 106, 3, 31, 2,
  255, 3, 255, 127, 255, 127, 223, 1, 18, 1, 0, 0, 31, 35, 95, 3, 242, 0, 9, 0, 255, 127, 234, 3,
  31, 1, 0, 0, 159, 41, 26, 0, 12, 0, 0, 0, 255, 127, 127, 2, 31, 0, 0, 0, 255, 127, 224, 3, 6, 2,
  32, 1, 255, 127, 235, 126, 31, 0, 0, 124, 255, 127, 255, 63, 0, 126, 31, 0, 255, 127, 255, 3, 31,
  0, 0, 0, 255, 3, 31, 0, 12, 0, 0, 0, 255, 127, 63, 3, 147, 1, 0, 0, 0, 0, 0, 66, 127, 3, 255,
  127, 255, 127, 140, 126, 0, 124, 0, 0, 255, 127, 239, 27, 128, 97, 0, 0, 255, 127, 0, 124, 224,
  3, 31, 124, 31, 0, 255, 3, 64, 65, 66, 32, 33, 34, 128, 129, 130, 16, 17, 18, 18, 176, 121, 184,
  173, 22, 23, 7, 186, 5, 124, 19, 0, 0, 0, 0,
];