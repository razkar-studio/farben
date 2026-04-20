use criterion::{black_box, criterion_group, criterion_main, Criterion};
use farben_core::{
    ansi::{Color, Ground, NamedColor, color_to_ansi, emphasis_to_ansi},
    lexer::{EmphasisType, TagType, Token, tokenize},
    parser::render,
    registry::insert_style,
};

// --- tokenize ---

fn bench_tokenize_plain(c: &mut Criterion) {
    c.bench_function("tokenize plain", |b| {
        b.iter(|| tokenize(black_box("hello world, no markup here at all")))
    });
}

fn bench_tokenize_complex(c: &mut Criterion) {
    c.bench_function("tokenize complex", |b| {
        b.iter(|| {
            tokenize(black_box(
                "[bold red]error:[/] something [bg:blue]went[/] wrong with [italic]this[/] call",
            ))
        })
    });
}

// --- render ---

fn make_tokens() -> Vec<Token> {
    vec![
        Token::Tag(TagType::Emphasis(EmphasisType::Bold)),
        Token::Tag(TagType::Color {
            color: Color::Named(NamedColor::Red),
            ground: Ground::Foreground,
        }),
        Token::Text("error:".into()),
        Token::Tag(TagType::ResetAll),
        Token::Text(" something ".into()),
        Token::Tag(TagType::Color {
            color: Color::Named(NamedColor::Blue),
            ground: Ground::Background,
        }),
        Token::Text("went".into()),
        Token::Tag(TagType::ResetAll),
        Token::Text(" wrong".into()),
    ]
}

fn bench_render(c: &mut Criterion) {
    c.bench_function("render", |b| {
        b.iter(|| render(black_box(make_tokens())))
    });
}

// --- full pipeline ---

fn bench_pipeline(c: &mut Criterion) {
    c.bench_function("pipeline", |b| {
        b.iter(|| {
            let tokens = tokenize(black_box(
                "[bold red]error:[/] something [bg:blue]went[/] wrong with [italic]this[/] call",
            ))
            .unwrap();
            render(tokens)
        })
    });
}

// --- ANSI encoding ---

fn bench_emphasis_to_ansi(c: &mut Criterion) {
    c.bench_function("emphasis_to_ansi", |b| {
        b.iter(|| emphasis_to_ansi(black_box(&EmphasisType::Bold)))
    });
}

fn bench_color_to_ansi_named(c: &mut Criterion) {
    c.bench_function("color_to_ansi named", |b| {
        b.iter(|| {
            color_to_ansi(
                black_box(&Color::Named(NamedColor::Red)),
                black_box(Ground::Foreground),
            )
        })
    });
}

fn bench_color_to_ansi_rgb(c: &mut Criterion) {
    c.bench_function("color_to_ansi rgb", |b| {
        b.iter(|| {
            color_to_ansi(
                black_box(&Color::Rgb(255, 128, 0)),
                black_box(Ground::Foreground),
            )
        })
    });
}

// --- registry lookup (via tokenize) ---

fn bench_registry_via_tokenize(c: &mut Criterion) {
    use farben_core::ansi::Style;
    insert_style("bench_danger", Style::parse("[bold red]").unwrap()).unwrap();
    c.bench_function("registry lookup via tokenize", |b| {
        b.iter(|| tokenize(black_box("[bench_danger]text")))
    });
}

// --- escape sequences ---

fn bench_tokenize_escape_open(c: &mut Criterion) {
    c.bench_function("tokenize [[ escape", |b| {
        b.iter(|| tokenize(black_box("use [[bold] to make text bold")))
    });
}

fn bench_tokenize_escape_symmetric(c: &mut Criterion) {
    c.bench_function("tokenize [[...]] symmetric escape", |b| {
        b.iter(|| tokenize(black_box("[[bold]]")))
    });
}

fn bench_tokenize_escape_mixed(c: &mut Criterion) {
    c.bench_function("tokenize mixed escapes and tags", |b| {
        b.iter(|| {
            tokenize(black_box(
                "[bold red]error:[/] use [[red] for red, [[bold] for bold",
            ))
        })
    });
}

fn bench_tokenize_bare_close_bracket(c: &mut Criterion) {
    c.bench_function("tokenize bare ] in text", |b| {
        b.iter(|| tokenize(black_box("result[0] = value")))
    });
}

fn bench_tokenize_double_close_bracket(c: &mut Criterion) {
    c.bench_function("tokenize ]] escape", |b| {
        b.iter(|| tokenize(black_box("array syntax: [[0]]")))
    });
}

criterion_group!(
    benches,
    bench_tokenize_plain,
    bench_tokenize_complex,
    bench_render,
    bench_pipeline,
    bench_emphasis_to_ansi,
    bench_color_to_ansi_named,
    bench_color_to_ansi_rgb,
    bench_registry_via_tokenize,
    bench_tokenize_escape_open,
    bench_tokenize_escape_symmetric,
    bench_tokenize_escape_mixed,
    bench_tokenize_bare_close_bracket,
    bench_tokenize_double_close_bracket,
);
criterion_main!(benches);
