# Installation & First Sound

## Installation

Add `tunes` to your `Cargo.toml`:

```toml
[dependencies]
tunes = "0.7.0"
```

### Platform-Specific Requirements

**Linux users** need ALSA development libraries:
```bash
# Debian/Ubuntu
sudo apt install libasound2-dev

# Fedora/RHEL
sudo dnf install alsa-lib-devel
```

**macOS and Windows** work out of the box with no additional dependencies.

---

## Three Levels of Introduction

We'll build up from a simple proof-of-concept to the kind of algorithmic music that makes Tunes unique:

1. **[Level 1: Your First Sound](./first-sound.md)** - A simple 440Hz tone (proof of life)
2. **[Level 2: Making Music](./making-music.md)** - Musical chord progression
3. **[Level 3: Algorithmic Music](./algorithmic.md)** - Collatz sequence melody

Each level introduces new concepts while building on the previous one.

Let's start with Level 1 →
