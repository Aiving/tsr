use super::{parse_ident, parse_literal};
use crate::{
    ast::{ImportClause, ImportDeclaration, ImportSpecifier},
    tags::{
        as_tag, brace_close_tag, brace_open_tag, comma_tag, from_tag, import_tag, positioned,
        semi_tag, star_tag, type_tag,
    },
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, terminated, tuple},
};

use tsr_lexer::{
    globals::{Positioned, TokenResult},
    tokens::Tokens,
};

pub fn parse_import_declaration(input: Tokens) -> TokenResult<Positioned<ImportDeclaration>> {
    map(
        positioned(delimited(
            import_tag,
            pair(
                opt(terminated(
                    alt((
                        map(
                            tuple((
                                brace_open_tag,
                                separated_list0(
                                    comma_tag,
                                    map(
                                        tuple((
                                            opt(type_tag),
                                            parse_ident,
                                            opt(tuple((as_tag, parse_ident))),
                                        )),
                                        |(type_only, ident, alias)| {
                                            type_only
                                                .as_ref()
                                                .map(|ty| ty.span)
                                                .unwrap_or(ident.span)
                                                .between(
                                                    alias
                                                        .as_ref()
                                                        .map(|(_, alias)| alias.span)
                                                        .unwrap_or(ident.span),
                                                )
                                                .wrap(ImportSpecifier {
                                                    is_type_only: type_only
                                                        .map(|ty| ty.wrap(true))
                                                        .unwrap_or(ident.wrap(false)),
                                                    property_name: alias
                                                        .as_ref()
                                                        .map(|_| ident.clone()),
                                                    name: alias
                                                        .map(|(start, alias)| {
                                                            start.between(&alias).wrap(alias.value)
                                                        })
                                                        .unwrap_or(ident),
                                                })
                                        },
                                    ),
                                ),
                                brace_close_tag,
                            )),
                            |(start, imports, end)| {
                                start
                                    .between(&end)
                                    .wrap(ImportClause::NamedImports(imports))
                            },
                        ),
                        map(
                            tuple((star_tag, as_tag, parse_ident)),
                            |(start, _, ident)| {
                                start
                                    .between(&ident)
                                    .wrap(ImportClause::NamespaceImport(ident))
                            },
                        ),
                        map(parse_ident, |ident| {
                            ident.span.wrap(ImportClause::Named(ident))
                        }),
                    )),
                    from_tag,
                )),
                parse_literal,
            ),
            semi_tag,
        )),
        |Positioned {
             value: (import_clause, module_specifier),
             span,
         }| {
            span.wrap(ImportDeclaration {
                import_clause,
                module_specifier,
            })
        },
    )(input)
}
