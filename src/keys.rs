/// Physical location of a key on the keyboard. Names and descriptions are taken
/// from the [W3C `KeyboardEvent` `code` values spec][1]. All "required" values
/// are included with the following exceptions:
///
/// - `Unidentified` is excluded; it is represented using [`Option::None`]
///   instead.
/// - `Meta` has been renamed to `Logo` for consistency with winit.
///
/// Use this when the physical location of the key is important, for example
/// when detecting WASD keys for movement controls in a game.
///
/// [1]: https://w3c.github.io/uievents-code/#key-code-attribute-value
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key {
    /// <kbd>`~</kbd> on a US keyboard. This is the <kbd>半角/全角/漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\|</kbd> (on the 101-key layout) and also for
    /// the key located between the <kbd>&quot;</kbd> and <kbd>Enter</kbd> keys on
    /// row C of the 102-, 104- and 106-key layouts. Labelled <kbd>#~</kbd> on a
    /// UK (102) keyboard.
    Backslash,
    /// <kbd>[{</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]}</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,&lt;</kbd> on a US keyboard.
    Comma,
    /// <kbd>0)</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1!</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2@</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3#</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4$</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5%</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6^</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7&amp;</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8*</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9(</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=+</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labelled <kbd>\|</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labelled <kbd>\ろ</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys. Labelled
    /// <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\/</kbd> on a Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard. Labelled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard. Labelled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard. Labelled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard. Labelled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard. Labelled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and
    /// <kbd>y</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>_</kbd> (underscore) on a US keyboard.
    Minus,
    /// <kbd>.&gt;</kbd> on a US keyboard.
    Period,
    /// <kbd>&#39;</kbd> on a US keyboard.
    Quote,
    /// <kbd>;:</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/?</kbd> on a US keyboard.
    Slash,

    /// <kbd>Alt</kbd>, <kbd>Option</kbd> or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd> or <kbd>⌥</kbd>. This is labelled
    /// <kbd>AltGr</kbd> key on many keyboard layouts.
    AltRight,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// The <kbd>Windows</kbd>, <kbd>⌘</kbd>, <kbd>Command</kbd> or other OS
    /// symbol key. Also called `MetaLeft`.
    LogoLeft,
    /// The <kbd>Windows</kbd>, <kbd>⌘</kbd>, <kbd>Command</kbd> or other OS
    /// symbol key. Also called `MetaRight`.
    LogoRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,

    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>. Labelled <kbd>Delete</kbd> on
    /// Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the
    /// right logo key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labelled <kbd>Return</kbd> on Apple
    /// keyboards.
    Enter,
    /// <kbd>&nbsp;</kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,

    /// Japanese: <kbd>変換</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ/ひらがな/ローマ字</kbd> (katakana/hiragana/romaji)
    KanaMode,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,

    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// <kbd>⌦</kbd>. The forward delete key. Note that on Apple keyboards, the
    /// key labelled <kbd>Delete</kbd> on the main part of the keyboard should
    /// be encoded as `Backspace`.
    Delete,
    /// <kbd>End</kbd> or <kbd>↘</kbd>
    End,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd> or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd> or <kbd>⇞</kbd>
    PageUp,

    /// On the Mac, the `NumLock` code should be used for the numpad
    /// <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard
    Numpad4,
    /// <kbd>5</kbd> on a keyboard
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>.</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    ///
    NumpadEnter,
    /// <kbd>\*</kbd> on a keyboard. For use with numpads that provide
    /// mathematical operations (<kbd>+</kbd>, <kbd>-</kbd>, <kbd>*</kbd> and
    /// <kbd>/</kbd>).
    NumpadMultiply,
    /// <kbd>-</kbd>
    NumpadSubtract,

    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>F1</kbd>
    F1,
    /// <kbd>F2</kbd>
    F2,
    /// <kbd>F3</kbd>
    F3,
    /// <kbd>F4</kbd>
    F4,
    /// <kbd>F5</kbd>
    F5,
    /// <kbd>F6</kbd>
    F6,
    /// <kbd>F7</kbd>
    F7,
    /// <kbd>F8</kbd>
    F8,
    /// <kbd>F9</kbd>
    F9,
    /// <kbd>F10</kbd>
    F10,
    /// <kbd>F11</kbd>
    F11,
    /// <kbd>F12</kbd>
    F12,
    /// <kbd>F13</kbd>
    F13,
    /// <kbd>F14</kbd>
    F14,
    /// <kbd>F15</kbd>
    F15,
    /// <kbd>F16</kbd>
    F16,
    /// <kbd>F17</kbd>
    F17,
    /// <kbd>F18</kbd>
    F18,
    /// <kbd>F19</kbd>
    F19,
    /// <kbd>F20</kbd>
    F20,
    /// <kbd>F21</kbd>
    F21,
    /// <kbd>F22</kbd>
    F22,
    /// <kbd>F23</kbd>
    F23,
    /// <kbd>F24</kbd>
    F24,
    /// Help. Not present on standard PC keyboards.
    Help,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
}
