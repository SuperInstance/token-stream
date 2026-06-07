# token-stream

Lexical token stream with position tracking, lookahead, and span management for compiler infrastructure.

## Features

- **Position tracking** — Line, column, and byte offset
- **Span management** — Source ranges with merge, overlap, and containment
- **Token types** — Identifiers, literals, operators, keywords, EOF
- **Lookahead buffer** — Peek ahead without consuming tokens
- **Token stream** — Cursor-based navigation with seek
- **Zero dependencies** — Pure `std` implementation

## License

MIT
