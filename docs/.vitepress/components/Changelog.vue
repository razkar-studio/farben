<template>
    <div class="changelog">
        <div
            v-for="release in releases"
            :key="release.version + release.crate"
            class="release"
        >
            <div class="release-header">
                <div class="release-title">
                    <span class="release-version">{{ release.version }}</span>
                    <span
                        class="release-crate"
                        :class="crateClass(release.crate)"
                        >{{ release.crate }}</span
                    >
                </div>
                <div class="release-meta">
                    <span v-if="release.subtitle" class="release-subtitle">{{
                        release.subtitle
                    }}</span>
                    <span class="release-date">{{ release.date }}</span>
                </div>
            </div>
            <div
                v-for="section in release.sections"
                :key="section.type"
                class="section"
            >
                <span :class="['section-badge', section.type.toLowerCase()]">{{
                    section.type
                }}</span>
                <ul class="section-items">
                    <li
                        v-for="(item, i) in section.items"
                        :key="i"
                        v-html="item"
                    ></li>
                </ul>
            </div>
        </div>
    </div>
</template>

<script setup>
function crateClass(crate) {
    if (crate === "farben-core") return "crate-core";
    if (crate === "farben-core (cascade)") return "crate-cascade";
    if (crate === "farben-macros") return "crate-macros";
    if (crate === "farben") return "crate-farben";
    if (crate === "farben / farben-macros") return "crate-macros";
    if (crate === "Global") return "crate-global";
    if (crate === "farben-md") return "crate-md";
}

const releases = [
    {
        version:
            "farben-build 0.1.0 / farben-macros 0.5.0 / farben-core 0.10.0 / farben-md 0.2.0 / farben 0.14.0",
        crate: "Global",
        subtitle: "Compile-time Custom Style Support",
        date: "2026-04-05",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>farben-build</code> 0.1.0 - a new build script helper crate. Call <code>farben_build::run()</code> from <code>build.rs</code> to read <code>farben.frb.toml</code> and generate two artifacts in <code>OUT_DIR</code>: <code>farben_styles.rs</code> (a Rust source file containing an <code>init_styles()</code> function that registers all styles and prefixes at runtime via <code>load_styles!()</code>) and <code>farben_registry.lsv</code> (a line-separated values file consumed by <code>farben-macros</code> proc macros at compile time). Use <code>farben_build::run_with(&amp;[paths])</code> to supply custom config file paths instead of the default. Config files use a TOML format with <code>[styles]</code> and <code>[prefixes]</code> sections. Namespaced sections like <code>[styles.myns]</code> produce keys of the form <code>myns:key</code>.",
                    "Absolutely zero external dependencies added.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>farben-macros</code> bumped to <code>0.5.0</code>. Every proc macro invocation now calls <code>load_registry()</code> at startup, which reads <code>farben_registry.lsv</code> from <code>OUT_DIR</code> and pre-populates the compile-time registry. As a result, <code>color!</code>, <code>colorb!</code>, and <code>validate_color!</code> now resolve user-defined style tags (e.g. <code>[myTag]</code>) that were declared in a <code>.frb.toml</code> config file.",
                    "<code>farben-core</code> bumped to <code>0.10.0</code>. No new public API.",
                    "<code>farben-md</code> bumped to <code>0.2.0</code>, picking up the <code>farben-core 0.10.0</code> dependency. No new public API.",
                ],
            },
        ],
    },
    {
        version: "0.9.0",
        crate: "farben-core",
        date: "2026-04-04",
        sections: [
            {
                type: "Added",
                items: [
                    "Lossy degrading. When the terminal does not support 24-bit true color, RGB values are automatically degraded to the nearest ANSI256 color. When the terminal only supports basic ANSI colors (8/16 colors), RGB and ANSI256 values are degraded to the nearest named color. The degrader module uses the <code>COLORTERM</code> and <code>TERM</code> environment variables to detect terminal color capabilities at runtime.",
                ],
            },
        ],
    },
    {
        version: "0.13.0",
        crate: "farben",
        date: "2026-04-04",
        sections: [
            {
                type: "Changed",
                items: [
                    "<code>farben-core</code> dependency bumped to <code>0.9.0</code>, picking up lossy degrading support.",
                ],
            },
        ],
    },
    {
        version: "0.12.0",
        crate: "farben",
        date: "2026-04-04",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>cwrite!</code>, <code>cwriteln!</code>, <code>cwriteb!</code>, <code>cwritebln!</code> - writer variants of the colored print macros. Work with any <code>Write</code> implementor. Useful for writing to files, <code>String</code> buffers, or custom writers. All four support the same markup features as the stdout variants (named colors, RGB, ANSI256, emphasis, bleeding via the <code>b</code> variants).",
                ],
            },
        ],
    },
    {
        version: "0.8.0",
        crate: "farben-core",
        date: "2026-04-04",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>RegistryError</code> enum - a separate error type for registry operations (<code>set_prefix</code>, <code>insert_style</code>). Split out from <code>LexError</code> because registry errors have no source position (they occur outside markup parsing). Has one variant: <code>UnknownStyle(String)</code>.",
                    "<code>LexErrorDisplay&lt;'a&gt;</code> struct - wraps a <code>&amp;LexError</code> and the original <code>&amp;str</code> input to produce compiler-style diagnostic output. Renders two lines: the full input string, then a caret (<code>^</code>) aligned to the byte offset of the error.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "All <code>LexError</code> variants now carry a <code>position: usize</code> field (byte offset into the markup string). Affected variants: <code>UnclosedTag</code>, <code>InvalidTag</code>, <code>UnclosedValue</code>, <code>InvalidArgumentCount</code>, <code>InvalidValue</code>, <code>InvalidResetTarget</code>. Previously no variants stored position info.",
                    "<code>LexError::UnknownStyle</code> removed - registry errors now use <code>RegistryError::UnknownStyle</code> instead.",
                    "<code>LexError</code>'s <code>Display</code> impl now includes position in every message (e.g. <code>\"invalid tag 'foo' at position 5\"</code>).",
                ],
            },
        ],
    },
    {
        version: "0.11.0",
        crate: "farben",
        date: "2026-04-04",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>farben::prelude</code> module - the recommended import path going forward. <code>use farben::prelude::*</code> brings every user-facing item into scope (functions, macros, types) gated by the same feature flags as their definitions. Prefer this over <code>use farben::*</code>, which also pulls in <code>color_runtime</code> and <code>validate_color</code> - items that are <code>pub</code> only to satisfy macro expansion, not intended for direct use.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>farben-core</code> dependency bumped to <code>0.8.0</code>, picking up position-aware <code>LexError</code> variants and the new <code>LexErrorDisplay</code> diagnostic formatter. <code>try_color</code> error messages now include the byte offset of the offending token.",
                    "All documentation and examples updated to use <code>use farben::prelude::*</code>.",
                ],
            },
        ],
    },
    {
        version: "0.10.0",
        crate: "farben",
        date: "2026-04-04",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>ansi_strip!(...)</code> macro that accepts <code>format!</code>-style arguments, builds the string, then strips all CSI ANSI escape sequences from the result. Non-CSI <code>ESC</code> bytes pass through unchanged. Returns <code>String</code>.",
                    "<code>strip_ansi</code> re-exported at the <code>farben</code> crate root from <code>farben-core::strip::strip_ansi</code>. Available via <code>use farben::*</code>.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>farben-core</code> dependency bumped to <code>0.7.0</code>.",
                ],
            },
        ],
    },
    {
        version: "0.7.0",
        crate: "farben-core",
        date: "2026-04-04",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>env</code> module: runtime detection of whether ANSI color output should be enabled. Respects <code>NO_COLOR</code> and <code>FORCE_COLOR</code> environment variable conventions, then falls back to TTY detection. Result is computed once per process and cached via <code>OnceLock</code>. TTY detection via <code>isatty(1)</code> on Unix, <code>GetConsoleMode</code> on Windows, <code>false</code> elsewhere.",
                    "<code>strip</code> module: <code>strip_ansi(input)</code> removes CSI ANSI escape sequences (<code>ESC [ ... &lt;letter&gt;</code>) from a string and returns plain text. Non-CSI <code>ESC</code> bytes are passed through unchanged.",
                    "Zero dependencies added to core library internals.",
                ],
            },
        ],
    },
    {
        version: "0.9.0",
        crate: "farben",
        date: "2026-03-20",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>ceprint!</code>, <code>ceprintln!</code>, <code>ceprintb!</code>, <code>ceprintbln!</code>, stderr variants of the colored print macros.",
                    "<code>mdeprint!</code>, <code>mdeprintln!</code>, stderr variants of the inline markdown print macros.",
                    "Empty invocation support for all print macros. <code>cprintln!()</code> now prints a bare newline, <code>cprint!()</code> prints nothing. Applies to <code>ceprint!</code>, <code>ceprintln!</code>, <code>mdprint!</code>, <code>mdprintln!</code>, <code>mdeprint!</code>, and all bleed variants.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>src/lib.rs</code> split into focused modules: <code>functions.rs</code>, <code>macros/color.rs</code>, <code>macros/format.rs</code>, <code>macros/markdown.rs</code>, <code>macros/eprint.rs</code>, and <code>tests.rs</code>. No public API changes.",
                ],
            },
        ],
    },
    {
        version: "0.8.3",
        crate: "farben",
        date: "2026-03-20",
        sections: [
            {
                type: "Fixed",
                items: [
                    "<code>prefix!</code> macro no longer required users to manually add <code>farben-core</code> as a dependency. The <code>set_prefix</code> function is now re-exported through <code>farben</code> and the macro expands via <code>farben::set_prefix</code>.",
                    "<code>color_fmt!</code>, <code>cprint!</code>, <code>cprintln!</code>, <code>cprintb!</code>, and <code>cprintbln!</code> (compile feature) no longer require <code>farben-macros</code> as a direct dependency. <code>validate_color</code> is now re-exported through <code>farben</code>",
                ],
            },
        ],
    },
    {
        version: "0.1.2",
        crate: "farben-md",
        date: "2026-03-17",
        sections: [
            {
                type: "Fixed",
                items: [
                    "Unclosed delimiters (<code>**</code>, <code>*</code>, <code>_</code>, <code>__</code>, <code>~~</code>) incorrectly produced styled tokens instead of falling back to plain text. <code>tokenize_inner</code> return type changed from <code>Vec&lt;MdToken&gt;</code> to <code>(Vec&lt;MdToken&gt;, bool)</code> to distinguish a found closing delimiter from end-of-input exhaustion.",
                    "Added <code>tokens_to_text</code> helper to reconstruct plain text from partially parsed token trees when a closing delimiter is never found.",
                ],
            },
            {
                type: "Added",
                items: [
                    "Full unit test suite for <code>lexer.rs</code> and <code>renderer.rs</code> covering plain text, all six token types, nesting, unclosed delimiters, empty input, mixed content, and consecutive spans.",
                ],
            },
        ],
    },
    {
        version: "0.8.1 / 0.6.3 / 0.4.1 / 0.1.1",
        crate: "Global",
        subtitle: "License Update",
        date: "2026-03-17",
        sections: [
            { type: "Changed", items: ["Changed LICENSE to MIT/Apache-2.0"] },
        ],
    },
    {
        version: "0.1.0",
        crate: "farben-md",
        date: "2026-03-17",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>tokenize()</code> - parses inline markdown into a recursive <code>MdToken</code> tree. Supports <code>**bold**</code>, <code>*italic*</code>, <code>_italic_</code>, <code>__underline__</code>, <code>~~strikethrough~~</code>, and <code>`inline code`</code>. Unclosed delimiters are treated as plain text.",
                    "<code>render()</code> - converts an <code>MdToken</code> tree into an ANSI-escaped string. Nested spans are handled via an active style stack that re-emits surviving styles after each reset.",
                    "<code>MdToken</code> - recursive token enum. <code>Text(String)</code> and <code>Code(String)</code> are leaf nodes; <code>Bold</code>, <code>Italic</code>, <code>Underline</code>, and <code>Strikethrough</code> carry <code>Vec&lt;MdToken&gt;</code>.",
                ],
            },
        ],
    },
    {
        version: "0.4.0",
        crate: "farben-macros",
        date: "2026-03-17",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>markdown!()</code> - proc macro that parses and renders inline markdown at compile time, emitting the final ANSI-escaped string baked into the binary. Enabled via the <code>markdown</code> feature.",
                ],
            },
        ],
    },
    {
        version: "0.8.0",
        crate: "farben",
        date: "2026-03-17",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>markdown()</code> - runtime function that parses and renders inline markdown into an ANSI-escaped string. Always succeeds. Enabled via the <code>markdown</code> feature.",
                    "<code>md_fmt!()</code> - renders inline markdown with format arguments. Always runtime. Enabled via the <code>markdown</code> feature.",
                    "<code>mdprint!()</code> - prints inline markdown to stdout without a newline. Runtime under <code>markdown</code>, compile-time under <code>markdown-compile</code>.",
                    "<code>mdprintln!()</code> - prints inline markdown to stdout with a trailing newline. Runtime under <code>markdown</code>, compile-time under <code>markdown-compile</code>.",
                    "<code>markdown</code> feature - enables runtime markdown rendering via <code>farben-md</code>.",
                    "<code>markdown-compile</code> feature - enables both <code>markdown</code> and <code>compile</code> with compile-time rendering via <code>farben-macros</code>.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>style!()</code> and <code>prefix!()</code> macros moved from <code>farben-core</code> to <code>farben</code>, under the <code>format</code> feature flag.",
                ],
            },
        ],
    },
    {
        version: "0.8.0 / 0.4.0 / 0.1.0",
        crate: "Global",
        date: "2026-03-17",
        subtitle: "Markdown Update",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>farben-md</code> - new crate providing inline markdown tokenization and ANSI rendering. Depends on <code>farben-core</code> for ANSI encoding.",
                    "<code>color_to_ansi()</code> and <code>emphasis_to_ansi()</code> made public in <code>farben-core</code> 0.6.2, enabling <code>farben-md</code> to delegate ANSI encoding without reimplementing it.",
                ],
            },
        ],
    },
    {
        version: "0.6.2",
        crate: "farben-core",
        date: "2026-03-17",
        sections: [
            {
                type: "Changed",
                items: [
                    "Made <code>style_to_ansi()</code> and <code>emphasis_to_ansi()</code> public functions.",
                ],
            },
        ],
    },
    {
        version: "0.7.1",
        crate: "farben",
        date: "2026-03-16",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>style!()</code> - moved from <code>farben-core</code>. Defines a named style in the global registry. Gated behind the <code>format</code> feature.",
                    "<code>prefix!()</code> - moved from <code>farben-core</code>. Sets a prefix string on a previously defined named style. Gated behind the <code>format</code> feature.",
                ],
            },
        ],
    },
    {
        version: "0.6.1",
        crate: "farben-core",
        date: "2026-03-16",
        sections: [
            {
                type: "Changed",
                items: [
                    "<code>style!()</code> - moved to <code>farben</code>. Users importing directly from <code>farben-core</code> should update to <code>farben::style!()</code>.",
                    "<code>prefix!()</code> - moved to <code>farben</code>. Users importing directly from <code>farben-core</code> should update to <code>farben::prefix!()</code>.",
                ],
            },
        ],
    },
    {
        version: "0.3.1",
        crate: "farben-macros",
        date: "2026-03-16",
        sections: [
            {
                type: "Changed",
                items: ["Minor dependency update to <code>farben-core</code>."],
            },
        ],
    },
    {
        version: "0.3.0",
        crate: "farben-macros",
        date: "2026-03-16",
        sections: [
            {
                type: "Changed",
                items: [
                    "<code>colorb!</code> - replaced one-line stub doc with a full doc comment explaining what bleed means, when to use it, and how it differs from <code>color!</code>. Includes a working example.",
                    "<code>validate_color!</code> - removed misleading user-facing example. Doc comment now explicitly marks it as internal and directs users toward <code>color!</code> and <code>color_fmt!</code>.",
                ],
            },
        ],
    },
    {
        version: "0.7.0",
        crate: "farben",
        date: "2026-03-16",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>colorb</code> - added missing doc comment explaining bleed behavior and when to use it over <code>color</code>.",
                    "<code>color_fmt!</code> (compile-time variant) - added missing doc comment. Previously the runtime variant was documented but the <code>compile</code>-feature counterpart had none.",
                ],
            },
            {
                type: "Changed",
                items: [
                    'Crate-level doc comment revised: fixed grammar and capitalized "German".',
                ],
            },
            {
                type: "Fixed",
                items: [
                    "<code>cprint!</code> (compile-time variant) - example referenced unbound variable <code>message</code>. Added declaration so it compiles as a doctest.",
                    "<code>cprintln!</code> (compile-time variant) - example referenced unbound variable <code>result</code>. Added declaration so it compiles as a doctest.",
                    '<code>test_try_color_inline_reset</code> - strengthened assertion to a full equality check against the expected output <code>"\\x1b[31mbefore\\x1b[0mafter\\x1b[0m"</code>.',
                ],
            },
        ],
    },
    {
        version: "0.6.0",
        crate: "farben-core",
        date: "2026-03-16",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>LexError::InvalidResetTarget</code> - returned when a reset tag targets something that cannot be reset (e.g. <code>[//]</code> or <code>[/prefix]</code>). Previously caused a panic.",
                    "<code>LexError::UnknownStyle</code> - returned by <code>registry::set_prefix</code> when the given style name has not been registered.",
                    "<code>registry::set_prefix</code> now returns <code>Result&lt;(), LexError&gt;</code> instead of <code>()</code>, allowing callers to handle unknown style names without panicking.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>ansi::style_to_ansi</code> promoted from <code>pub(crate)</code> to <code>pub</code>. Users building on top of <code>farben-core</code> can now call it directly.",
                    "<code>ansi::style_to_ansi</code> - removed <code>#[allow(unused)]</code> now that it is part of the public API.",
                    "<code>registry::prefix!</code> macro updated to call <code>.expect()</code> on the <code>Result</code> returned by <code>set_prefix</code>, preserving panic-on-misuse at the macro callsite.",
                    '<code>LexError::InvalidArgumentCount</code> display improved from <code>"expected N, got M"</code> to <code>"expected N arguments, got M"</code>.',
                ],
            },
            {
                type: "Fixed",
                items: [
                    "<code>lexer::parse_part</code> - replaced <code>panic!</code> with <code>Err(LexError::InvalidResetTarget)</code> when a reset tag targets a <code>Reset</code> or <code>Prefix</code> node.",
                    "<code>registry::set_prefix</code> - replaced <code>panic!</code> with <code>Err(LexError::UnknownStyle)</code> when the style name is not found in the registry.",
                    '<code>errors.rs</code> - corrected typo in <code>UnclosedValue</code> display: "parantheses" → "parentheses".',
                    '<code>ansi::NamedColor</code> doc comment - corrected "eight standard ANSI named colors" to "sixteen ANSI named colors".',
                    "<code>ansi::style_to_ansi</code> - added a working doctest demonstrating bold + named color output.",
                    "<code>parser::render</code> - removed unnecessary <code>.as_str()</code> calls on <code>String</code> return values.",
                ],
            },
        ],
    },
    {
        version: "0.6.0",
        crate: "farben-core (cascade)",
        date: "2026-03-16",
        sections: [
            {
                type: "Added",
                items: [
                    "Specified resets - <code>[/bold]</code>, <code>[/red]</code>, <code>[/italic]</code> etc. reset only the named style, leaving all other active styles intact.",
                    "<code>TagType::Reset</code> now takes <code>Option&lt;Box&lt;TagType&gt;&gt;</code> - <code>None</code> for full reset <code>[/]</code>, <code>Some(tag)</code> for partial reset.",
                    "<code>parser::render</code> now maintains an active tag stack, re-emitting surviving styles after a partial reset.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>TagType::Reset</code> changed from a unit variant to <code>Reset(Option&lt;Box&lt;TagType&gt;&gt;)</code>.",
                    "All existing <code>[/]</code> full reset behavior is preserved via <code>Reset(None)</code>.",
                ],
            },
            {
                type: "Fixed",
                items: [
                    "Fixed a bug where <code>colorb!()</code> did not exist when using the <code>compile</code> feature.",
                ],
            },
        ],
    },
    {
        version: "0.2.6",
        crate: "farben / farben-macros",
        date: "2026-03-16",
        sections: [
            {
                type: "Changed",
                items: ["Minor dependency update to <code>farben-core</code>."],
            },
        ],
    },
    {
        version: "0.5.1",
        crate: "farben",
        date: "2026-03-16",
        sections: [
            {
                type: "Changed",
                items: ["Minor dependency update to <code>farben-core</code>."],
            },
        ],
    },
    {
        version: "0.2.5",
        crate: "farben-macros",
        date: "2026-03-15",
        sections: [
            {
                type: "Changed",
                items: ["Minor dependency update to <code>farben-core</code>."],
            },
        ],
    },
    {
        version: "0.4.2",
        crate: "farben-core",
        date: "2026-03-15",
        sections: [
            {
                type: "Added",
                items: [
                    "Bright ANSI color variants (<code>bright-black</code> through <code>bright-white</code>).",
                ],
            },
        ],
    },
    {
        version: "0.5.0",
        crate: "farben-core (cascade)",
        date: "2026-03-15",
        sections: [
            {
                type: "Added",
                items: [
                    "Added bugs.",
                    "<code>Style::prefix</code> field - optional text prepended before the style's ANSI codes when applied.",
                    "<code>TagType::Prefix(String)</code> variant - carries prefix text through the token pipeline to the renderer.",
                    "<code>set_prefix()</code> - sets the prefix on an existing registry entry, panics if the style is not found.",
                    "<code>prefix!()</code> macro - user-facing API for binding a text prefix to a named style.",
                    "<code>style_to_tags()</code> now emits <code>TagType::Prefix</code> as the first tag when a prefix is present.",
                    "<code>parser::render()</code> now handles <code>TagType::Prefix</code> by appending the text directly to output.",
                    "<code>format</code> default feature - gates logic for <code>style!()</code> and <code>prefix!()</code>.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>style_to_tags()</code> no longer returns early on <code>reset</code> when a prefix is present; prefix is always emitted first.",
                    "<code>style!()</code> is now gated to the <code>format</code> feature.",
                ],
            },
        ],
    },
    {
        version: "0.4.1",
        crate: "farben-core",
        date: "2026-03-15",
        sections: [
            {
                type: "Fixed",
                items: [
                    "Bug where <code>prefix!()</code> interfered with actual color styling.",
                ],
            },
        ],
    },
    {
        version: "0.2.4",
        crate: "farben-macros",
        date: "2026-03-15",
        sections: [
            {
                type: "Changed",
                items: ["Minor dependency update to <code>farben-core</code>."],
            },
        ],
    },
    {
        version: "0.2.3",
        crate: "farben-macros",
        date: "2026-03-15",
        sections: [
            {
                type: "Changed",
                items: ["Minor dependency update to <code>farben-core</code>."],
            },
        ],
    },
    {
        version: "0.4.0",
        crate: "farben-core (cascade)",
        date: "2026-03-15",
        subtitle: "Public Farben Update",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>Style::parse()</code> - builds a <code>Style</code> from a farben markup string.",
                    "<code>Style::reset</code> field - when <code>true</code>, overrides all other style attributes with a full SGR reset.",
                    "<code>registry</code> module - global style registry backed by <code>OnceLock&lt;Mutex&lt;HashMap&lt;String, Style&gt;&gt;&gt;</code>.",
                    "<code>insert_style()</code> - inserts a named style into the global registry.",
                    "<code>search_registry()</code> - looks up a named style from the global registry.",
                    "<code>style!()</code> macro - user-facing API for defining custom named styles.",
                    "<code>style_to_tags()</code> - converts a <code>Style</code> into a <code>Vec&lt;TagType&gt;</code> for lexer expansion.",
                    "Custom tag resolution in <code>parse_part()</code> - unknown tags now check the registry before returning <code>InvalidTag</code>.",
                    "<code>parse_part()</code> return type changed from <code>Result&lt;TagType, LexError&gt;</code> to <code>Result&lt;Vec&lt;TagType&gt;, LexError&gt;</code> to support style expansion.",
                    "<code>colorb!()</code> bleeds at compile-time with the <code>colorb()</code> runtime counterpart.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>parse_tag()</code> updated to flatten nested <code>Vec&lt;TagType&gt;</code> results from <code>parse_part()</code>.",
                    "All functions using <code>compile</code> can now benefit from compile-time processing instead of just validation.",
                ],
            },
            {
                type: "Fixed",
                items: ["<code>color!()</code> now auto-resets."],
            },
        ],
    },
    {
        version: "0.2.2",
        crate: "farben-macros",
        date: "2026-03-15",
        sections: [
            {
                type: "Added",
                items: [
                    "Dependency update to <code>farben-core</code>.",
                    "<code>colorb!()</code> macro that bleeds.",
                ],
            },
            {
                type: "Fixed",
                items: ["<code>color!()</code> now auto-resets."],
            },
        ],
    },
    {
        version: "0.3.0",
        crate: "farben-core",
        date: "2026-03-15",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>Style::parse()</code> - builds a <code>Style</code> from a farben markup string.",
                    "<code>Style::reset</code> field - when <code>true</code>, overrides all other style attributes with a full SGR reset.",
                    "<code>registry</code> module - global style registry backed by <code>OnceLock&lt;Mutex&lt;HashMap&lt;String, Style&gt;&gt;&gt;</code>.",
                    "<code>insert_style()</code> - inserts a named style into the global registry.",
                    "<code>search_registry()</code> - looks up a named style from the global registry.",
                    "<code>style!()</code> macro - user-facing API for defining custom named styles.",
                    "<code>style_to_tags()</code> - converts a <code>Style</code> into a <code>Vec&lt;TagType&gt;</code> for lexer expansion.",
                    "Custom tag resolution in <code>parse_part()</code> - unknown tags now check the registry before returning <code>InvalidTag</code>.",
                    "<code>parse_part()</code> return type changed from <code>Result&lt;TagType, LexError&gt;</code> to <code>Result&lt;Vec&lt;TagType&gt;, LexError&gt;</code> to support style expansion.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>parse_tag()</code> updated to flatten nested <code>Vec&lt;TagType&gt;</code> results from <code>parse_part()</code>.",
                ],
            },
        ],
    },
    {
        version: "0.3.3",
        crate: "farben",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>cprintb!</code> and <code>cprintbln!</code> for color-bleeding printing.",
                ],
            },
        ],
    },
    {
        version: "0.2.1",
        crate: "farben-macros",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: ["100% documentation coverage."],
            },
        ],
    },
    {
        version: "0.2.1",
        crate: "farben-core",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: ["100% documentation coverage."],
            },
        ],
    },
    {
        version: "0.3.2",
        crate: "farben",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>cprint!()</code> - prints farben-colored markup to stdout without a newline, behaves like <code>print!</code>.",
                    "<code>cprintln!()</code> - prints farben-colored markup to stdout with a trailing newline, behaves like <code>println!</code>.",
                    "Both macros support format args and compile-time validation when the <code>compile</code> feature is enabled.",
                ],
            },
        ],
    },
    {
        version: "0.3.0",
        crate: "farben",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>color!()</code> - compile-time markup processing via optional <code>compile</code> feature flag.",
                    "<code>color_fmt!()</code> - format args support with compile-time tag validation when <code>compile</code> is enabled.",
                    "<code>validate_color!()</code> - proc-macro that validates farben markup at compile time.",
                    "<code>color_runtime()</code> - internal runtime fallback used by <code>color_fmt!</code>.",
                    "<code>bg:</code> and <code>fg:</code> prefix support in color tags - <code>[bg:red]</code>, <code>[fg:white bg:blue]</code>.",
                    "<code>farben-core</code> and <code>farben-macros</code> as separate workspace crates.",
                    "<code>compile</code> feature flag for opt-in compile-time processing.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "Internal logic moved to <code>farben-core</code>.",
                    "<code>color()</code> replaced by <code>color!</code> proc-macro when <code>compile</code> feature is enabled.",
                ],
            },
        ],
    },
    {
        version: "0.2.0",
        crate: "farben-core",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>bg:</code> and <code>fg:</code> prefix support for color tags.",
                    "<code>Ground</code> field added to <code>TagType::Color</code> variant.",
                    "Background color support in <code>encode_color_sgr</code> and <code>color_to_ansi</code>.",
                    "New tests for background color parsing, tokenizing, and rendering.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>TagType::Color(Color)</code> restructured to <code>TagType::Color { color: Color, ground: Ground }</code>.",
                ],
            },
        ],
    },
    {
        version: "0.2.0",
        crate: "farben-core (cascade)",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>color!()</code> - compile-time markup processing via optional <code>compile</code> feature flag.",
                    "<code>color_fmt!()</code> - format args support with compile-time tag validation.",
                    "<code>validate_color!()</code> - proc-macro that validates farben markup at compile time, emitting the original string literal unchanged on success.",
                    "<code>color_runtime()</code> - internal runtime fallback used by <code>color_fmt!</code>.",
                    "<code>farben-core</code> - extracted shared logic crate containing lexer, parser, ANSI encoding, and error types.",
                    "<code>farben-macros</code> - proc-macro crate powering compile-time processing.",
                    "Cargo workspace setup with <code>farben</code>, <code>farben-core</code>, and <code>farben-macros</code> as members.",
                    "<code>compile</code> feature flag - opt-in compile-time processing via <code>farben-macros</code>.",
                ],
            },
            {
                type: "Changed",
                items: [
                    "<code>color()</code> is now replaced by the <code>color!</code> proc-macro when the <code>compile</code> feature is enabled.",
                    "Internal logic extracted from <code>farben</code> into <code>farben-core</code> for shared use across crates.",
                ],
            },
        ],
    },
    {
        version: "0.1.0",
        crate: "farben-core (cascade)",
        date: "2026-03-14",
        sections: [
            {
                type: "Added",
                items: [
                    "<code>color()</code> - colorizes a string using markup-like syntax, panics on invalid markup.",
                    "<code>try_color()</code> - same as <code>color()</code> but returns <code>Result&lt;String, LexError&gt;</code>.",
                    "Named color tags: <code>[black]</code>, <code>[red]</code>, <code>[green]</code>, <code>[yellow]</code>, <code>[blue]</code>, <code>[magenta]</code>, <code>[cyan]</code>, <code>[white]</code>.",
                    "256-color palette support via <code>[ansi(n)]</code>.",
                    "24-bit RGB support via <code>[rgb(r,g,b)]</code>.",
                    "Emphasis tags: <code>[bold]</code>, <code>[dim]</code>, <code>[italic]</code>, <code>[underline]</code>, <code>[blink]</code>, <code>[strikethrough]</code>.",
                    "Multi-tag brackets: <code>[bold red]</code>, <code>[italic rgb(255,0,0)]</code>.",
                    "Reset tag <code>[/]</code> to clear all active styles.",
                    "Escape sequence <code>\\[</code> to treat <code>[</code> as a literal character.",
                    "<code>LexError</code> with variants <code>UnclosedTag</code>, <code>InvalidTag</code>, <code>InvalidValue</code>, <code>InvalidArgumentCount</code>.",
                    "Foreground and background color support via <code>Ground</code> enum.",
                    "Automatic reset appended to all <code>color()</code> and <code>try_color()</code> output.",
                ],
            },
        ],
    },
];
</script>

<style scoped>
.changelog {
    max-width: 740px;
    margin: 0 auto;
    padding: 2rem 0 4rem;
}

.release {
    padding-bottom: 2rem;
    margin-bottom: 2rem;
    border-bottom: 1px solid var(--vp-c-divider);
}

.release:last-child {
    border-bottom: none;
}

.release-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1.25rem;
}

.release-title {
    display: flex;
    align-items: baseline;
    gap: 0.6rem;
}

.release-version {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--vp-c-brand-1);
}

.release-crate {
    font-size: 0.72rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    padding: 0.2em 0.6em;
    border-radius: 4px;
}

.crate-core {
    background-color: var(--vp-c-brand-soft);
    color: var(--vp-c-brand-1);
}

.crate-macros {
    background-color: var(--vp-c-purple-soft);
    color: var(--vp-c-purple-1);
}

.crate-farben {
    background-color: var(--vp-c-indigo-soft);
    color: var(--vp-c-indigo-1);
}

.crate-cascade {
    background-color: var(--vp-c-teal-soft);
    color: var(--vp-c-teal-1);
}

.crate-md {
    background-color: var(--vp-c-cyan-soft);
    color: var(--vp-c-cyan-1);
}

.crate-global {
    background-color: var(--vp-c-default-soft);
    color: var(--vp-c-text-2);
}

.release-meta {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
}

.release-subtitle {
    font-size: 0.9rem;
    font-style: italic;
    color: var(--vp-c-text-2);
}

.release-date {
    font-size: 0.9rem;
    color: var(--vp-c-text-3);
}

.section {
    margin-bottom: 1.1rem;
}

.section-badge {
    display: inline-block;
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    padding: 0.2em 0.55em;
    border-radius: 4px;
    margin-bottom: 0.5rem;
}

.section-badge.added {
    background-color: var(--vp-c-green-soft);
    color: var(--vp-c-green-1);
}

.section-badge.changed {
    background-color: var(--vp-c-yellow-soft);
    color: var(--vp-c-yellow-1);
}

.section-badge.fixed {
    background-color: var(--vp-c-red-soft);
    color: var(--vp-c-red-1);
}

.section-items {
    margin: 0;
    padding-left: 1.25rem;
    list-style: disc;
}

.section-items li {
    font-size: 1rem;
    line-height: 1.75;
    color: var(--vp-c-text-1);
    padding: 0.15rem 0;
}

.section-items li + li {
    margin-top: 0.25rem;
}
</style>
