/// Color Palette
///
/// Paletton palette saved [here](http://paletton.com/#uid=71q1F0k++Tfkg++qe++++iU++ci)
///
/// Note that '_' prefixing the names prevents `#[warn(dead_code)]` at compile time
///
/// YELLOW           100, 82.7, 0
/// YELLOW_LIGHT     100, 89, 36.9
/// YELLOW_DARK      59.2, 49, 0
///
/// GREEN            0.0, 92.2, 54.9
/// GREEN_LIGHT      18.0, 1.0, 67.1
/// GREEN_VERY_LIGHT 36.9, 100, 74.5
/// GREEN_DARK       0.0, 40.0, 23.9
///
/// RED              100, 29.8, 0
/// RED_LIGHT        100, 42.4, 18
/// RED_DARK         38.4, 11.4, 0
///
/// BLUE             22, 0, 92.5
/// BLUE_LIGHT       44.7, 27.5, 100
/// BLUE_DARK        6.3, 0, 26.3


pub const _BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const _T_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.5];
pub const _WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub const _GREEN: [f32; 4] = [0.0, 0.922, 0.549, 1.0];
pub const _L_GREEN: [f32; 4] = [0.18, 1.0, 0.671, 1.0];
pub const _VL_GREEN: [f32; 4] = [0.369, 1.0, 0.745, 1.0];
pub const _D_GREEN: [f32; 4] = [0.0, 0.4, 0.239, 1.0];

pub const _YELLOW: [f32; 4] = [1.0, 0.827, 0.0, 1.0];
pub const _L_YELLOW: [f32; 4] = [1.0, 0.890, 0.369, 1.0];
pub const _D_YELLOW: [f32; 4] = [0.592, 0.490, 0.0, 1.0];

pub const _RED: [f32; 4] = [1.0, 0.298, 0.0, 1.0];
pub const _L_RED: [f32; 4] = [1.0, 0.424, 0.18, 1.0];
pub const _D_RED: [f32; 4] = [0.384, 0.114, 0.0, 1.0];

pub const _BLUE: [f32; 4] = [22.0, 0.0, 92.5, 1.0];
pub const _L_BLUE: [f32; 4] = [44.7, 27.5, 1.0, 1.0];
pub const _D_BLUE: [f32; 4] = [6.3, 0.0, 26.3, 1.0];

pub const _SNAKE_PRIMARY: [f32; 4] = _GREEN;
pub const _SNAKE_HEAD: [f32; 4] = _D_GREEN;
pub const _SNAKE_TAIL: [f32; 4] = _L_GREEN;

pub const _GAME_BG: [f32; 4] = _VL_GREEN;
pub const _GAME_BORDER: [f32; 4] = _D_RED;

pub const _TEXT: [f32; 4] = _L_YELLOW;
pub const _TEXT_ALT: [f32; 4] = _L_GREEN;

pub const _FOOD_PRIMARY: [f32; 4] = _BLUE;
pub const _FOOD_SECONDARY: [f32; 4] = _L_BLUE;
