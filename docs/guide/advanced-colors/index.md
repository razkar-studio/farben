# Advanced Color Formats

Beyond named colors and basic RGB, Farben supports six additional color spaces: HSL, HSV (also known as HSB), HWB, Lab, LCH, and OKLCh. All of them accept optional spaces inside the parentheses and validate argument ranges.

## HSL

Hue, Saturation, Lightness. Matches the CSS `hsl()` function.

```
[hsl(h, s, l)]
```

- `h`: 0-360 (hue angle in degrees)
- `s`: 0-100 (saturation percent)
- `l`: 0-100 (lightness percent)

```rust
use farben::prelude::*;

cprintln!("[hsl(0,100,50)]Pure red via HSL.");
cprintln!("[hsl(120,100,50)]Pure green via HSL.");
cprintln!("[hsl(240,100,50)]Pure blue via HSL.");
cprintln!("[hsl(45,80,60)]A warm golden color.");
```

## HSV / HSB

Hue, Saturation, Value (also known as Hue, Saturation, Brightness). Both names work interchangeably.

```
[hsv(h, s, v)] or [hsb(h, s, b)]
```

- `h`: 0-360 (hue angle in degrees)
- `s`: 0-100 (saturation percent)
- `v`/`b`: 0-100 (value/brightness percent)

```rust
use farben::prelude::*;

cprintln!("[hsv(0,100,100)]Pure red via HSV.");
cprintln!("[hsv(200,80,90)]A vivid sky blue.");
cprintln!("[hsb(300,60,80)]A muted magenta via HSB.");
```

## HWB

Hue, Whiteness, Blackness. An intuitive model: start with a pure hue, add white, add black.

```
[hwb(h, w, b)]
```

- `h`: 0-360 (hue angle in degrees)
- `w`: 0-100 (whiteness percent)
- `b`: 0-100 (blackness percent)
- Constraint: `w + b` must not exceed 100

```rust
use farben::prelude::*;

cprintln!("[hwb(0,0,0)]Pure red.");
cprintln!("[hwb(0,50,0)]Pink (red + white).");
cprintln!("[hwb(0,0,50)]Dark red (red + black).");
cprintln!("[hwb(200,30,20)]A muted teal.");
```

## Lab

CIE L\*a\*b\* is a perceptually uniform color space. L\* is lightness, a\* is green-red, b\* is blue-yellow.

```
[lab(l, a, b)]
```

- `l`: 0-100 (lightness)
- `a`: -128 to 127 (green to red)
- `b`: -128 to 127 (blue to yellow)

```rust
use farben::prelude::*;

cprintln!("[lab(0,0,0)]Black.");
cprintln!("[lab(100,0,0)]White.");
cprintln!("[lab(53,80,67)]A vivid red.");
cprintln!("[lab(50,-50,0)]A cool green.");
```

## LCH

CIE L\*C\*h\* (Lightness, Chroma, Hue) is a cylindrical representation of Lab.

```
[lch(l, c, h)]
```

- `l`: 0-100 (lightness)
- `c`: 0-150 (chroma / saturation)
- `h`: 0-360 (hue angle in degrees)

```rust
use farben::prelude::*;

cprintln!("[lch(50,30,270)]A blue hue.");
cprintln!("[lch(70,50,45)]A warm orange.");
```

## OKLCh

A newer, more perceptually uniform variant of LCH. Uses the same cylindrical model but with a different lightness scale.

```
[oklch(l, c, h)]
```

- `l`: 0.0-1.0 (lightness, as a decimal fraction)
- `c`: 0.0-0.4 (chroma / saturation)
- `h`: 0-360 (hue angle in degrees)

```rust
use farben::prelude::*;

cprintln!("[oklch(0.6,0.15,280)]A cool purple.");
cprintln!("[oklch(0.8,0.2,120)]A vibrant green.");
```

## Hex Colors

Short and long hex notation.

```
[#rgb] or [#rrggbb]
```

3-digit shorthand (`#rgb`) doubles each channel: `#f00` becomes `#ff0000`.

```rust
use farben::prelude::*;

cprintln!("[#ff0000]Pure red via hex.");
cprintln!("[#00ff00]Pure green via hex.");
cprintln!("[#f00]Shorthand hex red.");
cprintln!("[#0f0]Shorthand hex green.");
cprintln!("[#ff8800]A warm orange via hex.");
```

### Validation

All color formats validate their arguments. If a value is out of range or the wrong number of arguments is given, `color()` panics and `try_color()` returns an error:

```rust
use farben::try_color;

// Invalid: H out of range
assert!(try_color("[hsl(400,50,50)]text").is_err());

// Invalid: missing argument
assert!(try_color("[hsl(100,50)]text").is_err());

// Invalid: w + b > 100
assert!(try_color("[hwb(0,60,60)]text").is_err());

// Invalid: hex with wrong length
assert!(try_color("[#ff00]text").is_err());

// Invalid: hex with non-hex characters
assert!(try_color("[#xyz]text").is_err());
```

## Summary

| Format | Syntax | Ranges |
|--------|--------|--------|
| HSL | `[hsl(h,s,l)]` | H:0-360, S:0-100, L:0-100 |
| HSV / HSB | `[hsv(h,s,v)]` / `[hsb(h,s,b)]` | H:0-360, S:0-100, V/B:0-100 |
| HWB | `[hwb(h,w,b)]` | H:0-360, W:0-100, B:0-100, W+B<=100 |
| Lab | `[lab(l,a,b)]` | L:0-100, A:-128-127, B:-128-127 |
| LCH | `[lch(l,c,h)]` | L:0-100, C:0-150, H:0-360 |
| OKLCh | `[oklch(l,c,h)]` | L:0.0-1.0, C:0.0-0.4, H:0-360 |
| Hex | `[#rgb]` / `[#rrggbb]` | 0-9, a-f, A-F |
