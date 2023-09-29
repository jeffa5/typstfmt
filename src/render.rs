use std::unreachable;

use tracing::debug;
use typst::syntax::{ast::*, SyntaxKind, SyntaxNode};

use crate::{writer::Writer, Config};

/// Renderer that has the information for writing out.
pub struct Renderer {
    pub writer: Writer,
}

impl Renderer {
    /// Render the AST from the given node.
    pub fn render(&mut self, node: SyntaxNode) {
        debug!(?node, "render");
        node.cast::<Markup>().unwrap().render(self)
    }

    /// Get the rendered value.
    pub fn finish(self) -> String {
        self.writer.finish()
    }

    fn config(&self) -> &Config {
        self.writer.config()
    }
}

/// Render all text from a general syntax node.
fn render_anon(node: &SyntaxNode, renderer: &mut Renderer) {
    debug!(?node, "render_anon");
    if let Some(space) = node.cast::<Space>() {
        space.render(renderer);
    } else if node.kind().is_grouping() {
        match node.kind() {
            SyntaxKind::LeftBrace | SyntaxKind::LeftParen | SyntaxKind::LeftBracket => {
                renderer.writer.open_grouping(node.text());
            }
            SyntaxKind::RightBrace | SyntaxKind::RightParen | SyntaxKind::RightBracket => {
                renderer.writer.close_grouping(node.text());
            }
            _ => unreachable!(),
        }
    } else if renderer.config().spacing && node.kind() == SyntaxKind::LineComment {
        renderer.writer.push(node.text());
        if renderer.config().multiline {
            renderer.writer.newline_with_indent();
        }
    } else if renderer.config().spacing && node.kind() == SyntaxKind::BlockComment {
        if node.text().contains('\n') {
            for line in node.text().lines() {
                let line = line.trim();
                if line.starts_with('*') {
                    // align the stars
                    if renderer.config().spacing {
                        renderer.writer.push(" ");
                    }
                }
                renderer.writer.push(line).newline_with_indent();
            }
        } else {
            renderer.writer.push(node.text());
        }
    } else {
        renderer.writer.push(node.text());
        for child in node.children() {
            render_anon(child, renderer)
        }
    }
}

/// An AstNode that we can render.
trait Renderable<'a>: AstNode<'a> + std::fmt::Debug + Clone {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_anon(self.clone().to_untyped(), renderer)
    }

    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        self.render_impl(renderer);
    }
}

fn render_typed_or_text<'a, T: Renderable<'a>>(node: &'a SyntaxNode, renderer: &mut Renderer) {
    if let Some(typed) = node.cast::<T>() {
        typed.render(renderer);
    } else {
        render_anon(node, renderer)
    }
}

fn render_typed_or_text_2<'a, T1: Renderable<'a>, T2: Renderable<'a>>(
    node: &'a SyntaxNode,
    renderer: &mut Renderer,
) {
    if let Some(typed) = node.cast::<T1>() {
        typed.render(renderer);
    } else if let Some(typed) = node.cast::<T2>() {
        typed.render(renderer);
    } else {
        render_anon(node, renderer)
    }
}

fn render_children_typed_or_text_untyped<'a, T: Renderable<'a>>(
    node: &'a SyntaxNode,
    renderer: &mut Renderer,
) {
    for child in node.children() {
        render_typed_or_text::<T>(child, renderer)
    }
}

fn render_children_typed_or_text<'a, T: Renderable<'a>>(
    node: &(impl AstNode<'a> + Clone),
    renderer: &mut Renderer,
) {
    render_children_typed_or_text_untyped::<T>(node.clone().to_untyped(), renderer)
}

fn render_children_typed_or_text_untyped_2<'a, T1: Renderable<'a>, T2: Renderable<'a>>(
    node: &'a SyntaxNode,
    renderer: &mut Renderer,
) {
    for child in node.children() {
        render_typed_or_text_2::<T1, T2>(child, renderer)
    }
}

fn render_children_typed_or_text_2<'a, T1: Renderable<'a>, T2: Renderable<'a>>(
    node: &(impl AstNode<'a> + Clone),
    renderer: &mut Renderer,
) {
    render_children_typed_or_text_untyped_2::<T1, T2>(node.clone().to_untyped(), renderer)
}

#[derive(Debug)]
struct Children<'a> {
    items: Vec<&'a SyntaxNode>,
    index: usize,
}

impl<'a> Children<'a> {
    fn new(node: &'a SyntaxNode) -> Self {
        Self {
            items: node.children().collect(),
            index: 0,
        }
    }

    fn next(&mut self) -> Option<&&SyntaxNode> {
        let item = self.items.get(self.index);
        self.index += 1;
        item
    }

    fn current(&self) -> Option<&&SyntaxNode> {
        if self.index > 0 {
            self.items.get(self.index - 1)
        } else {
            None
        }
    }

    fn has_next(&self, f: impl Fn(SyntaxKind) -> bool) -> bool {
        let mut index = self.index;
        loop {
            let item = self.items.get(index);
            index += 1;
            if let Some(it) = item {
                if f(it.kind()) {
                    return true;
                }
            } else {
                return false;
            }
        }
    }

    fn any(&self, f: impl FnMut(&&SyntaxNode) -> bool) -> bool {
        self.items.iter().any(f)
    }

    fn peek_next(&self) -> Option<&&SyntaxNode> {
        self.items.get(self.index)
    }

    fn peek_prev(&self) -> Option<&&SyntaxNode> {
        self.index.checked_sub(2).and_then(|i| self.items.get(i))
    }
}

impl<'a> Renderable<'a> for Markup<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while children.next().is_some() {
            let child = children.current().unwrap();
            if let Some(parbreak) = child.cast::<Parbreak>() {
                if children.peek_prev().is_some() && children.peek_next().is_some() {
                    parbreak.render(renderer);
                } else {
                    renderer.writer.newline_with_indent();
                }
            } else if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl<'a> Renderable<'a> for CodeBlock<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while let Some(child) = children.next() {
            if let Some(code) = child.cast::<Code>() {
                code.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl<'a> Renderable<'a> for Code<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl<'a> Renderable<'a> for Text<'a> {}
impl<'a> Renderable<'a> for Space<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        if renderer.config().spacing {
            let text = self.to_untyped().text();
            if text.contains("\n\n") {
                renderer.writer.parbreak();
            } else if text.contains('\n') {
                // convert newlines to newlines with indent
                renderer.writer.newline_with_indent();
            } else {
                renderer.writer.space();
            }
        } else {
            renderer.writer.push(self.to_untyped().text());
        }
    }
}
impl<'a> Renderable<'a> for Linebreak<'a> {}
impl<'a> Renderable<'a> for Parbreak<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        if renderer.config().spacing {
            renderer.writer.parbreak();
        } else {
            render_anon(self.to_untyped(), renderer)
        }
    }
}
impl<'a> Renderable<'a> for Escape<'a> {}
impl<'a> Renderable<'a> for Shorthand<'a> {}
impl<'a> Renderable<'a> for SmartQuote<'a> {}
impl<'a> Renderable<'a> for Strong<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Emph<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Raw<'a> {}
impl<'a> Renderable<'a> for Link<'a> {}
impl<'a> Renderable<'a> for Label<'a> {}
impl<'a> Renderable<'a> for Ref<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<ContentBlock>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Heading<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl<'a> Renderable<'a> for ListItem<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while let Some(child) = children.next() {
            if child.kind() == SyntaxKind::ListMarker {
                render_anon(child, renderer);
                if renderer.config().spacing {
                    renderer.writer.space();
                }
                renderer.writer.inc_indent();
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
        renderer.writer.dec_indent();
    }
}
impl<'a> Renderable<'a> for EnumItem<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while let Some(child) = children.next() {
            if child.kind() == SyntaxKind::EnumMarker {
                render_anon(child, renderer);
                if renderer.config().spacing {
                    renderer.writer.space();
                }
                renderer.writer.inc_indent();
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
        renderer.writer.dec_indent();
    }
}
impl<'a> Renderable<'a> for TermItem<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while let Some(child) = children.next() {
            if child.kind() == SyntaxKind::TermMarker {
                render_anon(child, renderer);
                if renderer.config().spacing {
                    renderer.writer.space();
                }
                renderer.writer.inc_indent();
            } else if child.kind() == SyntaxKind::Colon {
                renderer.writer.push(":");
                if renderer.config().spacing {
                    renderer.writer.space();
                }
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
        renderer.writer.dec_indent();
    }
}

impl<'a> Renderable<'a> for Equation<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Math>(self, renderer)
    }
}

impl<'a> Renderable<'a> for Math<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl<'a> Renderable<'a> for MathIdent<'a> {}
impl<'a> Renderable<'a> for MathAlignPoint<'a> {}
impl<'a> Renderable<'a> for MathDelimited<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text_2::<Expr, Math>(self, renderer)
    }
}
impl<'a> Renderable<'a> for MathAttach<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for MathFrac<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for MathRoot<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for MathPrimes<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Ident<'a> {}
impl<'a> Renderable<'a> for None<'a> {}
impl<'a> Renderable<'a> for Auto<'a> {}
impl<'a> Renderable<'a> for Bool<'a> {}
impl<'a> Renderable<'a> for Int<'a> {}
impl<'a> Renderable<'a> for Float<'a> {}
impl<'a> Renderable<'a> for Numeric<'a> {}
impl<'a> Renderable<'a> for Str<'a> {}

impl<'a> Renderable<'a> for ContentBlock<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while let Some(child) = children.next() {
            if child.kind() == SyntaxKind::LeftBracket {
                renderer.writer.open_grouping(child.text());
            } else if child.kind() == SyntaxKind::RightBracket {
                renderer.writer.close_grouping(child.text());
            } else if let Some(markup) = child.cast::<Markup>() {
                markup.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for Parenthesized<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Array<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        // TODO: can't use ArrayItem instead of Expr here because the spread variant doesn't
        // include the dots.
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Dict<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        let multiline = renderer.config().multiline
            && children.any(|c| c.kind() == SyntaxKind::Space && c.text().contains('\n'));
        let past_argument = |children: &Children, renderer: &mut Renderer| {
            if multiline {
                renderer.writer.push(",").newline_with_indent();
            } else if children.has_next(|k| {
                !k.is_trivia()
                    && !k.is_grouping()
                    && k != SyntaxKind::ContentBlock
                    && k != SyntaxKind::Comma
            }) {
                renderer.writer.push(",");
                if renderer.config().spacing {
                    renderer.writer.push(" ");
                }
            }
        };
        while let Some(child) = children.next() {
            if let Some(expr) = child.cast::<Keyed>() {
                expr.render(renderer);
                past_argument(&children, renderer);
            } else if let Some(named) = child.cast::<Named>() {
                named.render(renderer);
                past_argument(&children, renderer);
            } else if let Some(spread) = child.cast::<Spread>() {
                spread.render(renderer);
                past_argument(&children, renderer);
            } else if multiline && child.kind() == SyntaxKind::LeftParen {
                renderer
                    .writer
                    .open_grouping(child.text())
                    .newline_with_indent();
            } else if child.kind() == SyntaxKind::Comma
                || (renderer.config().spacing && child.kind() == SyntaxKind::Space)
            {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for Unary<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for Binary<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else if renderer.config().spacing && BinOp::from_kind(child.kind()).is_some() {
                renderer.writer.push(" ").push(child.text()).push(" ");
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer)
            }
        }
    }
}
impl<'a> Renderable<'a> for FieldAccess<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text_2::<Expr, Ident>(self, renderer)
    }
}
impl<'a> Renderable<'a> for FuncCall<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if let Some(typed) = child.cast::<Expr>() {
                typed.render(renderer);
            } else if child.kind() == SyntaxKind::Args {
                render_args(child, renderer)
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl<'a> Renderable<'a> for Closure<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if let Some(typed) = child.cast::<Expr>() {
                typed.render(renderer);
            } else if child.kind() == SyntaxKind::Params {
                render_params(child, renderer);
            } else if matches!(child.kind(), SyntaxKind::Arrow | SyntaxKind::Eq) {
                if renderer.config().spacing {
                    renderer.writer.space();
                }
                renderer.writer.push(child.text());
                if renderer.config().spacing {
                    renderer.writer.space();
                }
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else if let Some(typed) = child.cast::<Ident>() {
                typed.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for LetBinding<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        while let Some(child) = children.next() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else if let Some(named) = child.cast::<Pattern>() {
                named.render(renderer);
            } else if child.kind() == SyntaxKind::Eq {
                if renderer.config().spacing
                    && !children.peek_prev().map_or(false, |n| {
                        n.kind() != SyntaxKind::Space && n.text().ends_with(' ')
                    })
                {
                    renderer.writer.push(" ");
                }
                renderer.writer.push("=");
                if renderer.config().spacing
                    && !children.peek_next().map_or(false, |n| {
                        n.kind() != SyntaxKind::Space && n.text().starts_with(' ')
                    })
                {
                    renderer.writer.push(" ");
                }
            } else if child.kind() == SyntaxKind::Let {
                renderer.writer.push("let");
                if renderer.config().spacing {
                    renderer.writer.push(" ");
                }
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for DestructAssignment<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text_2::<Pattern, Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for SetRule<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if child.kind() == SyntaxKind::Args {
                render_args(child, renderer)
            } else if let Some(typed) = child.cast::<Expr>() {
                typed.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for ShowRule<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for Conditional<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        let mut children = Children::new(self.to_untyped());
        let spacing = |children: &Children, renderer: &mut Renderer| {
            if renderer.config().multiline
                && children.peek_prev().map_or(false, |p| {
                    p.kind() == SyntaxKind::Space && p.text().contains('\n')
                })
            {
                renderer.writer.newline_with_indent();
            } else if renderer.config().spacing {
                renderer.writer.push(" ");
            }
        };
        while children.next().is_some() {
            // Get around the borrow checker
            let child = children.current().unwrap();
            if let Some(expr) = child.cast::<Expr>() {
                spacing(&children, renderer);
                expr.render(renderer);
            } else if child.kind() == SyntaxKind::Else {
                spacing(&children, renderer);
                renderer.writer.push("else");
            } else if child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for WhileLoop<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for ForLoop<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text_2::<Pattern, Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for ModuleImport<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if child.kind() == SyntaxKind::ImportItems {
                render_children_typed_or_text_untyped::<Ident>(child, renderer);
            } else if let Some(typed) = child.cast::<Expr>() {
                typed.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl<'a> Renderable<'a> for ModuleInclude<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl<'a> Renderable<'a> for LoopBreak<'a> {}
impl<'a> Renderable<'a> for LoopContinue<'a> {}
impl<'a> Renderable<'a> for FuncReturn<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl<'a> Renderable<'a> for Pattern<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        match self {
            Pattern::Normal(expr) => expr.render(renderer),
            Pattern::Placeholder(underscore) => render_anon(underscore.to_untyped(), renderer),
            Pattern::Destructuring(destructuring) => destructuring.render(renderer),
        }
    }
}

impl<'a> Renderable<'a> for Destructuring<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Ident>(self, renderer)
    }
}

impl<'a> Renderable<'a> for Named<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        for child in self.to_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else if let Some(ident) = child.cast::<Ident>() {
                ident.render(renderer);
            } else if child.kind() == SyntaxKind::Colon {
                renderer.writer.push(":");
                if renderer.config().spacing {
                    renderer.writer.push(" ");
                }
            } else if renderer.config().spacing && child.kind() == SyntaxKind::Space {
                // skip
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl<'a> Renderable<'a> for Spread<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text_2::<Expr, Ident>(self, renderer)
    }
}

impl<'a> Renderable<'a> for Keyed<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl<'a> Renderable<'a> for Arg<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        match self {
            Arg::Pos(expr) | Arg::Spread(expr) => {
                expr.render(renderer);
            }
            Arg::Named(named) => named.render(renderer),
        }
    }
}

impl<'a> Renderable<'a> for Param<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        match self {
            Param::Pos(pat) => pat.render(renderer),
            Param::Named(named) => named.render(renderer),
            Param::Sink(spread) => spread.render(renderer),
        }
    }
}

impl<'a> Renderable<'a> for ArrayItem<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        match self {
            ArrayItem::Pos(expr) | ArrayItem::Spread(expr) => expr.render(renderer),
        }
    }
}

impl<'a> Renderable<'a> for DictItem<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        match self {
            DictItem::Named(named) => named.render(renderer),
            DictItem::Keyed(keyed) => keyed.render(renderer),
            DictItem::Spread(expr) => expr.render(renderer),
        }
    }
}

impl<'a> Renderable<'a> for Expr<'a> {
    fn render_impl(&self, renderer: &mut Renderer) {
        match self {
            Expr::Text(node) => node.render(renderer),
            Expr::Space(node) => node.render(renderer),
            Expr::Linebreak(node) => node.render(renderer),
            Expr::Parbreak(node) => node.render(renderer),
            Expr::Escape(node) => node.render(renderer),
            Expr::Shorthand(node) => node.render(renderer),
            Expr::SmartQuote(node) => node.render(renderer),
            Expr::Strong(node) => node.render(renderer),
            Expr::Emph(node) => node.render(renderer),
            Expr::Raw(node) => node.render(renderer),
            Expr::Link(node) => node.render(renderer),
            Expr::Label(node) => node.render(renderer),
            Expr::Ref(node) => node.render(renderer),
            Expr::Heading(node) => node.render(renderer),
            Expr::List(node) => node.render(renderer),
            Expr::Enum(node) => node.render(renderer),
            Expr::Term(node) => node.render(renderer),
            Expr::Equation(node) => node.render(renderer),
            Expr::Math(node) => node.render(renderer),
            Expr::MathIdent(node) => node.render(renderer),
            Expr::MathAlignPoint(node) => node.render(renderer),
            Expr::MathDelimited(node) => node.render(renderer),
            Expr::MathAttach(node) => node.render(renderer),
            Expr::MathFrac(node) => node.render(renderer),
            Expr::MathRoot(node) => node.render(renderer),
            Expr::MathPrimes(node) => node.render(renderer),
            Expr::Ident(node) => node.render(renderer),
            Expr::None(node) => node.render(renderer),
            Expr::Auto(node) => node.render(renderer),
            Expr::Bool(node) => node.render(renderer),
            Expr::Int(node) => node.render(renderer),
            Expr::Float(node) => node.render(renderer),
            Expr::Numeric(node) => node.render(renderer),
            Expr::Str(node) => node.render(renderer),
            Expr::Code(node) => node.render(renderer),
            Expr::Content(node) => node.render(renderer),
            Expr::Parenthesized(node) => node.render(renderer),
            Expr::Array(node) => node.render(renderer),
            Expr::Dict(node) => node.render(renderer),
            Expr::Unary(node) => node.render(renderer),
            Expr::Binary(node) => node.render(renderer),
            Expr::FieldAccess(node) => node.render(renderer),
            Expr::FuncCall(node) => node.render(renderer),
            Expr::Closure(node) => node.render(renderer),
            Expr::Let(node) => node.render(renderer),
            Expr::DestructAssign(node) => node.render(renderer),
            Expr::Set(node) => node.render(renderer),
            Expr::Show(node) => node.render(renderer),
            Expr::Conditional(node) => node.render(renderer),
            Expr::While(node) => node.render(renderer),
            Expr::For(node) => node.render(renderer),
            Expr::Import(node) => node.render(renderer),
            Expr::Include(node) => node.render(renderer),
            Expr::Break(node) => node.render(renderer),
            Expr::Continue(node) => node.render(renderer),
            Expr::Return(node) => node.render(renderer),
        }
    }
}

// TODO: can't use ArrayItem instead of Expr here because the spread variant doesn't
// include the dots.
fn render_args(node: &SyntaxNode, renderer: &mut Renderer) {
    let mut children = Children::new(node);
    let multiline = renderer.config().multiline
        && children.any(|c| c.kind() == SyntaxKind::Space && c.text().contains('\n'));
    debug!(?node, ?multiline, "render_args");
    let mut in_parens = false;
    let past_argument = |children: &Children, renderer: &mut Renderer, in_parens: bool| {
        if in_parens {
            if multiline {
                renderer.writer.push(",").newline_with_indent();
            } else if children.has_next(|k| {
                !k.is_trivia()
                    && !k.is_grouping()
                    && k != SyntaxKind::ContentBlock
                    && k != SyntaxKind::Colon
            }) {
                renderer.writer.push(",");
                if renderer.config().spacing {
                    renderer.writer.push(" ");
                }
            }
        }
    };
    while let Some(child) = children.next() {
        if let Some(expr) = child.cast::<Expr>() {
            expr.render(renderer);
            past_argument(&children, renderer, in_parens);
        } else if let Some(named) = child.cast::<Named>() {
            named.render(renderer);
            past_argument(&children, renderer, in_parens);
        } else if let Some(spread) = child.cast::<Spread>() {
            spread.render(renderer);
            past_argument(&children, renderer, in_parens);
        } else if child.kind() == SyntaxKind::LeftParen {
            in_parens = true;
            if multiline {
                renderer
                    .writer
                    .open_grouping(child.text())
                    .newline_with_indent();
            } else {
                render_anon(child, renderer);
            }
        } else if child.kind() == SyntaxKind::RightParen {
            in_parens = false;
            render_anon(child, renderer);
        } else if child.kind() == SyntaxKind::Comma || child.kind() == SyntaxKind::Space {
            // skip
        } else {
            render_anon(child, renderer);
        }
    }
}

fn render_params(node: &SyntaxNode, renderer: &mut Renderer) {
    debug!(?node, "render_params");
    let mut children = Children::new(node);
    let multiline = renderer.config().multiline
        && children.any(|c| c.kind() == SyntaxKind::Space && c.text().contains('\n'));
    while let Some(child) = children.next() {
        if let Some(param) = child.cast::<Param>() {
            param.render(renderer);
            if multiline {
                renderer.writer.push(",").newline_with_indent();
            } else if children.has_next(|k| !k.is_trivia() && !k.is_grouping()) {
                renderer.writer.push(",");
                if renderer.config().spacing {
                    renderer.writer.push(" ");
                }
            }
        } else if multiline && child.kind() == SyntaxKind::LeftParen {
            renderer
                .writer
                .open_grouping(child.text())
                .newline_with_indent();
        } else if child.kind() == SyntaxKind::Comma || child.kind() == SyntaxKind::Space {
            // skip
        } else {
            render_anon(child, renderer);
        }
    }
}
