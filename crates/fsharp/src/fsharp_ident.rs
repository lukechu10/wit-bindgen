use heck::{ToLowerCamelCase, ToUpperCamelCase};

pub(crate) trait ToFSharpIdent: ToOwned {
    fn fsharp_keywords() -> &'static [&'static str];
    fn to_fsharp_ident(&self) -> Self::Owned;
    fn to_fsharp_ident_upper(&self) -> Self::Owned;
}

impl ToFSharpIdent for str {
    // Source: https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/keyword-reference
    fn fsharp_keywords() -> &'static [&'static str] {
        static FSHARP_KEY_WORDS: &[&str] = &[
            "abstract",
            "and",
            "as",
            "assert",
            "base",
            "begin",
            "class",
            "default",
            "delegate",
            "do",
            "done",
            "downcast",
            "downto",
            "elif",
            "else",
            "end",
            "exception",
            "extern",
            "false",
            "finally",
            "fixed",
            "for",
            "fun",
            "function",
            "global",
            "if",
            "in",
            "inherit",
            "inline",
            "interface",
            "internal",
            "lazy",
            "let",
            "match",
            "member",
            "module",
            "mutable",
            "namespace",
            "new",
            "not",
            "null",
            "of",
            "open",
            "or",
            "override",
            "private",
            "public",
            "rec",
            "return",
            "select",
            "static",
            "struct",
            "then",
            "to",
            "true",
            "try",
            "type",
            "upcast",
            "use",
            "val",
            "void",
            "when",
            "while",
            "with",
            "yield",
            "const",
            // Reserved tokens because they are keywords in OCaml.
            "asr",
            "land",
            "lor",
            "lsl",
            "lsr",
            "lxor",
            "mod",
            "sig",
            // Reserved tokens for future expansion of F#.
            "break",
            "checked",
            "component",
            "const",
            "constraint",
            "continue",
            "event",
            "external",
            "include",
            "mixin",
            "parallel",
            "process",
            "protected",
            "pure",
            "sealed",
            "tailcall",
            "trait",
            "virtual",
        ];
        FSHARP_KEY_WORDS
    }

    fn to_fsharp_ident(&self) -> String {
        // Escape F# keywords
        if Self::fsharp_keywords().contains(&self) {
            format!("@{}", self)
        } else {
            self.to_lower_camel_case()
        }
    }

    fn to_fsharp_ident_upper(&self) -> String {
        // Escape F# keywords
        if Self::fsharp_keywords().contains(&self) {
            format!("@{}", self)
        } else {
            self.to_upper_camel_case()
        }
    }
}
