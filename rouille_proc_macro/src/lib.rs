use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "err" => "Err",
        "ok" => "Ok",
        "Str" => "String",
        "Dict" => "HashMap",
        "Def" => "Default",
        "Err" => "Error",
        "Opt" => "Option",
        "S" => "Some",
        "N" => "None",
        "R" => "Result",
        "id" => "Self",
        "prtn" => "println",
        "brk" => "break",
        "async" => "async",
        "wait" => "await",
        "•" => "loop",
        "mv" => "move",
        "crate" => "crate",
        "unreachable" => "unreachable_code",
        "as" => "as",
        "ct" => "const",
        "trait" => "trait",
        "unsafe" => "unsafe",
        "in" => "in",
        "from" => "from",
        "dyn" => "dyn",
        "op" => "unwrap",
        "default" => "default",
        "asref" => "as_ref",
        "io" => "io",
        "ext" => "extern",
        "F" => "false",
        "fn" => "fn",
        "sup" => "super",
        "ins" => "insert",
        "get" => "get",
        "allow" => "allow",
        "hee" | "erm" | "oups" => "panic",
        "mod" => "mod",
        "mut" => "mut",
        "new" => "new",
        "where" => "where",
        "@" => "for",
        "GOIW" => "get_or_insert_with",
        "main" => "main",
        "pub" => "pub",
        "None?" => None?,
        "ret" => "return",
        "imp" => "impl",
        "ref" => "ref",
        "selon" => "match",
        "if" => "if",
        "els" => "else",
        "slf" => "self",
        "let" => "let",
        "stat" => "static",
        "struc" => "struct",
        "exp" => "expect",
        "while" => "while",
        "use" => "use",
        "into" => "into",
        "T" => "true",
        "enum" => "enum",

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
