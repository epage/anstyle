/// A set of text effects
///
/// # Examples
///
/// ```rust
/// let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// ```
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effects(u8);

impl Effects {
    const PLAIN: Self = Effects(0);

    pub const BOLD: Self = Effects(1 << 0);
    pub const DIMMED: Self = Effects(1 << 1);
    /// Not widely supported. Sometimes treated as inverse or blink
    pub const ITALIC: Self = Effects(1 << 2);
    /// Style extensions exist for Kitty, VTE, mintty and iTerm2.
    pub const UNDERLINE: Self = Effects(1 << 3);
    pub const BLINK: Self = Effects(1 << 4);
    /// Swap foreground and background colors; inconsistent emulation
    pub const INVERT: Self = Effects(1 << 5);
    pub const HIDDEN: Self = Effects(1 << 6);
    ///  Characters legible but marked as if for deletion. Not supported in Terminal.app
    pub const STRIKETHROUGH: Self = Effects(1 << 7);

    /// No effects enabled
    ///
    /// # Examples
    ///
    /// ```rust
    /// let effects = anstyle::Effects::new();
    /// ```
    pub const fn new() -> Self {
        Self::PLAIN
    }

    /// Check if no effects are enabled
    ///
    /// # Examples
    ///
    /// ```rust
    /// let effects = anstyle::Effects::new();
    /// assert!(effects.is_plain());
    ///
    /// let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
    /// assert!(!effects.is_plain());
    /// ```
    pub const fn is_plain(self) -> bool {
        self.0 == Self::PLAIN.0
    }

    /// Returns `true` if all of the effects in `other` are contained within `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
    /// assert!(effects.contains(anstyle::Effects::BOLD));
    ///
    /// let effects = anstyle::Effects::new();
    /// assert!(!effects.contains(anstyle::Effects::BOLD));
    /// ```
    #[inline(always)]
    pub const fn contains(self, other: Effects) -> bool {
        (other.0 & self.0) == other.0
    }

    /// Inserts the specified effects in-place.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let effects = anstyle::Effects::new().insert(anstyle::Effects::new());
    /// assert!(effects.is_plain());
    ///
    /// let effects = anstyle::Effects::new().insert(anstyle::Effects::BOLD);
    /// assert!(effects.contains(anstyle::Effects::BOLD));
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn insert(mut self, other: Effects) -> Self {
        self.0 |= other.0;
        self
    }

    /// Removes the specified effects in-place.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).remove(anstyle::Effects::BOLD);
    /// assert!(!effects.contains(anstyle::Effects::BOLD));
    /// assert!(effects.contains(anstyle::Effects::UNDERLINE));
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn remove(mut self, other: Effects) -> Self {
        self.0 &= !other.0;
        self
    }

    /// Reset all effects in-place
    /// ```rust
    /// let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).clear();
    /// assert!(!effects.contains(anstyle::Effects::BOLD));
    /// assert!(!effects.contains(anstyle::Effects::UNDERLINE));
    /// ```
    #[inline(always)]
    pub const fn clear(self) -> Self {
        Self::new()
    }

    /// Enable or disable the specified effects depending on the passed value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let effects = anstyle::Effects::new().set(anstyle::Effects::BOLD, true);
    /// assert!(effects.contains(anstyle::Effects::BOLD));
    /// ```
    #[inline]
    pub const fn set(self, other: Self, enable: bool) -> Self {
        if enable {
            self.insert(other)
        } else {
            self.remove(other)
        }
    }

    /// Iterate over enabled effects
    #[inline(always)]
    pub fn iter(self) -> EffectIter {
        EffectIter {
            index: 0,
            effects: self,
        }
    }

    /// Iterate over enabled effect indices
    #[inline(always)]
    pub(crate) fn index_iter(self) -> EffectIndexIter {
        EffectIndexIter {
            index: 0,
            effects: self,
        }
    }

    /// Render the ANSI code
    pub fn render(self) -> impl core::fmt::Display {
        EffectsDisplay(self)
    }
}

/// # Examples
///
/// ```rust
/// let effects = anstyle::Effects::new();
/// assert_eq!(format!("{:?}", effects), "Effects()");
///
/// let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// assert_eq!(format!("{:?}", effects), "Effects(BOLD | UNDERLINE)");
/// ```
impl core::fmt::Debug for Effects {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Effects(")?;
        for (i, index) in self.index_iter().enumerate() {
            if i != 0 {
                write!(f, " | ")?;
            }
            write!(f, "{}", METADATA[index].name)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

/// # Examples
///
/// ```rust
/// let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// assert_eq!(format!("{:?}", effects), "Effects(BOLD | UNDERLINE)");
/// ```
impl core::ops::BitOr for Effects {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        self.insert(rhs)
    }
}

/// # Examples
///
/// ```rust
/// let mut effects = anstyle::Effects::BOLD;
/// effects |= anstyle::Effects::UNDERLINE;
/// assert_eq!(format!("{:?}", effects), "Effects(BOLD | UNDERLINE)");
/// ```
impl core::ops::BitOrAssign for Effects {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = self.insert(other);
    }
}

/// # Examples
///
/// ```rust
/// let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE) - anstyle::Effects::BOLD;
/// assert_eq!(format!("{:?}", effects), "Effects(UNDERLINE)");
/// ```
impl core::ops::Sub for Effects {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.remove(other)
    }
}

/// # Examples
///
/// ```rust
/// let mut effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
/// effects -= anstyle::Effects::BOLD;
/// assert_eq!(format!("{:?}", effects), "Effects(UNDERLINE)");
/// ```
impl core::ops::SubAssign for Effects {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = self.remove(other);
    }
}

pub(crate) struct Metadata {
    pub(crate) name: &'static str,
    pub(crate) code: usize,
}

pub(crate) const METADATA: [Metadata; 8] = [
    Metadata {
        name: "BOLD",
        code: 1,
    },
    Metadata {
        name: "DIMMED",
        code: 2,
    },
    Metadata {
        name: "ITALIC",
        code: 3,
    },
    Metadata {
        name: "UNDERLINE",
        code: 4,
    },
    Metadata {
        name: "BLINK",
        code: 5,
    },
    Metadata {
        name: "INVERT",
        code: 7,
    },
    Metadata {
        name: "HIDDEN",
        code: 8,
    },
    Metadata {
        name: "STRIKETHROUGH",
        code: 9,
    },
];

struct EffectsDisplay(Effects);

impl core::fmt::Display for EffectsDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_plain() {
            return Ok(());
        }

        write!(f, "\x1B[")?;
        for (i, index) in self.0.index_iter().enumerate() {
            if i != 0 {
                write!(f, ";")?;
            }
            write!(f, "{}", METADATA[index].code)?;
        }
        write!(f, "m")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectIter {
    index: usize,
    effects: Effects,
}

impl Iterator for EffectIter {
    type Item = Effects;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < METADATA.len() {
            let index = self.index;
            self.index += 1;

            let effect = Effects(1 << index);
            if self.effects.contains(effect) {
                return Some(effect);
            }
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EffectIndexIter {
    index: usize,
    effects: Effects,
}

impl Iterator for EffectIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < METADATA.len() {
            let index = self.index;
            self.index += 1;

            let effect = Effects(1 << index);
            if self.effects.contains(effect) {
                return Some(index);
            }
        }

        None
    }
}
