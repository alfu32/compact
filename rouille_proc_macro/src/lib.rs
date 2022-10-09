use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "🖕" => "Err",
        "👍" => "Ok",
        "✍️" => "String",
        "📖" => "HashMap",
        "🛟" => "Default",
        "👎" => "Error",
        "👁️" => "Option",
        "☝️" => "Some",
        "👌" => "None",
        "💰" => "Result",
        "💡" => "Self",
        "🖨️" => "println",
        "💔" => "break",
        "😬" => "async",
        "😳" => "await",
        "🔄" => "loop",
        "🚀" => "move",
        "🎁" => "crate",
        "☕️" => "unreachable_code",
        "🥸" => "as",
        "🪨" => "const",
        "🎸" => "trait",
        "☠️" => "unsafe",
        "🌈" => "in",
        "💧" => "from",
        "🧨" => "dyn",
        "🤮" => "unwrap",
        "😀" => "default",
        "⚽️" => "as_ref",
        "🥕" => "io",
        "🦶" => "extern",
        "🐲" => "false",
        "🦾" => "fn",
        "☝️" => "super",
        "➕" => "insert",
        "🤔" => "get",
        "🤩" => "allow",
        "hee" | "erm" | "oups" => "panic",
        "🧳" => "mod",
        "⚙️" => "mut",
        "🥳" => "new",
        "🫙" => "where",
        "🎡" => "for",
        "GOIW" => "get_or_insert_with",
        "main" => "main",
        "🚃" => "pub",
        "🖤?" => None?,
        "💳" => "return",
        "💡" => "impl",
        "©️" => "ref",
        "🗝️" => "match",
        "☝️" => "if",
        "👇" => "else",
        "🏠" => "self",
        "🦄" => "let",
        "🗿" => "static",
        "🧱" => "struct",
        "🏴‍☠️" => "expect",
        "💨" => "while",
        "🧻" => "use",
        "🚽" => "into",
        "🦄" => "true",
        "🎧" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
