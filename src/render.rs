use regex::Regex;
use tracing::debug;
use typst::syntax::{ast::*, LinkedNode, SyntaxKind, SyntaxNode};

use crate::{writer::Writer, Config};

/// Renderer that has the information for writing out.
pub struct Renderer {
    pub writer: Writer,
}

impl Renderer {
    /// Render the AST from the given node.
    pub fn render(&mut self, node: LinkedNode) {
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
    } else {
        renderer.writer.push(&node.text());
        for child in node.children() {
            render_anon(child, renderer)
        }
    }
}

/// An AstNode that we can render.
trait Renderable: AstNode + std::fmt::Debug {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_anon(self.as_untyped(), renderer)
    }
}

fn render_typed_or_text<T: Renderable>(node: &SyntaxNode, renderer: &mut Renderer) {
    if let Some(typed) = node.cast::<T>() {
        typed.render(renderer);
    } else {
        render_anon(node, renderer)
    }
}

fn render_typed_or_text_2<T1: Renderable, T2: Renderable>(
    node: &SyntaxNode,
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

fn render_children_typed_or_text_untyped<T: Renderable>(
    node: &SyntaxNode,
    renderer: &mut Renderer,
) {
    for child in node.children() {
        render_typed_or_text::<T>(child, renderer)
    }
}

fn render_children_typed_or_text<T: Renderable>(node: &impl AstNode, renderer: &mut Renderer) {
    render_children_typed_or_text_untyped::<T>(node.as_untyped(), renderer)
}

fn render_children_typed_or_text_untyped_2<T1: Renderable, T2: Renderable>(
    node: &SyntaxNode,
    renderer: &mut Renderer,
) {
    for child in node.children() {
        render_typed_or_text_2::<T1, T2>(child, renderer)
    }
}

fn render_children_typed_or_text_2<T1: Renderable, T2: Renderable>(
    node: &impl AstNode,
    renderer: &mut Renderer,
) {
    render_children_typed_or_text_untyped_2::<T1, T2>(node.as_untyped(), renderer)
}

impl Renderable for Markup {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl Renderable for CodeBlock {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(code) = child.cast::<Code>() {
                code.render(renderer);
            } else if child.kind() == SyntaxKind::LeftBrace {
                renderer.writer.push("{").inc_indent();
            } else if child.kind() == SyntaxKind::RightBrace {
                renderer.writer.dec_indent().push("}");
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl Renderable for Code {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl Renderable for Text {}
impl Renderable for Space {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        if renderer.config().spacing {
            // collapse multiple spaces
            let regex = Regex::new(" +").unwrap();
            let s = regex
                .replace_all(self.as_untyped().text(), " ")
                .into_owned();
            // convert newlines to newlines with indent
            let s = s.replace("\n", &format!("\n{}", renderer.writer.current_indent()));
            renderer.writer.push(&s);
        } else {
            renderer.writer.push(self.as_untyped().text());
        }
    }
}
impl Renderable for Linebreak {}
impl Renderable for Parbreak {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        if renderer.config().spacing {
            renderer.writer.newline().newline_with_indent();
        } else {
            render_anon(self.as_untyped(), renderer)
        }
    }
}
impl Renderable for Escape {}
impl Renderable for Shorthand {}
impl Renderable for SmartQuote {}
impl Renderable for Strong {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl Renderable for Emph {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl Renderable for Raw {}
impl Renderable for Link {}
impl Renderable for Label {}
impl Renderable for Ref {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<ContentBlock>(self, renderer)
    }
}
impl Renderable for Heading {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl Renderable for ListItem {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl Renderable for EnumItem {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}
impl Renderable for TermItem {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Markup>(self, renderer)
    }
}

impl Renderable for Equation {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Math>(self, renderer)
    }
}

impl Renderable for Math {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl Renderable for MathIdent {}
impl Renderable for MathAlignPoint {}
impl Renderable for MathDelimited {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text_2::<Expr, Math>(self, renderer)
    }
}
impl Renderable for MathAttach {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for MathFrac {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for MathRoot {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for Ident {}
impl Renderable for None {}
impl Renderable for Auto {}
impl Renderable for Bool {}
impl Renderable for Int {}
impl Renderable for Float {}
impl Renderable for Numeric {}
impl Renderable for Str {}

impl Renderable for ContentBlock {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(markup) = child.cast::<Markup>() {
                markup.render(renderer);
            } else if child.kind() == SyntaxKind::LeftBracket {
                renderer.writer.push("[").inc_indent();
            } else if child.kind() == SyntaxKind::RightBracket {
                renderer.writer.dec_indent().push("]");
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl Renderable for Parenthesized {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for Array {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        // TODO: can't use ArrayItem instead of Expr here because the spread variant doesn't
        // include the dots.
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for Dict {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<DictItem>(self, renderer)
    }
}
impl Renderable for Unary {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for Binary {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else if BinOp::from_kind(child.kind()).is_some() && renderer.config().spacing {
                renderer.writer.push(" ").push(&child.text()).push(" ");
            } else {
                render_anon(child, renderer)
            }
        }
    }
}
impl Renderable for FieldAccess {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text_2::<Expr, Ident>(self, renderer)
    }
}
impl Renderable for FuncCall {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
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

impl Renderable for Closure {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(typed) = child.cast::<Expr>() {
                typed.render(renderer);
            } else if child.kind() == SyntaxKind::Params {
                render_params(child, renderer);
            } else if let Some(typed) = child.cast::<Ident>() {
                typed.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl Renderable for LetBinding {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        let children = self.as_untyped().children();
        for child in children {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else if let Some(named) = child.cast::<Pattern>() {
                named.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl Renderable for DestructAssignment {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text_2::<Pattern, Expr>(self, renderer)
    }
}
impl Renderable for SetRule {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
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
impl Renderable for ShowRule {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else {
                render_anon(child, renderer);
            }
        }
    }
}
impl Renderable for Conditional {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for WhileLoop {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for ForLoop {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text_2::<Pattern, Expr>(self, renderer)
    }
}
impl Renderable for ModuleImport {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
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
impl Renderable for ModuleInclude {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}
impl Renderable for LoopBreak {}
impl Renderable for LoopContinue {}
impl Renderable for FuncReturn {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl Renderable for Pattern {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        match self {
            Pattern::Normal(expr) => expr.render(renderer),
            Pattern::Placeholder(underscore) => render_anon(underscore.as_untyped(), renderer),
            Pattern::Destructuring(destructuring) => destructuring.render(renderer),
        }
    }
}

impl Renderable for Destructuring {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Ident>(self, renderer)
    }
}

impl Renderable for Named {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        for child in self.as_untyped().children() {
            if let Some(expr) = child.cast::<Expr>() {
                expr.render(renderer);
            } else if let Some(ident) = child.cast::<Ident>() {
                ident.render(renderer);
            } else if child.kind() == SyntaxKind::Colon && renderer.config().spacing {
                renderer.writer.push(": ");
            } else {
                render_anon(child, renderer);
            }
        }
    }
}

impl Renderable for Spread {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text_2::<Expr, Ident>(self, renderer)
    }
}

impl Renderable for Keyed {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        render_children_typed_or_text::<Expr>(self, renderer)
    }
}

impl Renderable for Arg {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        match self {
            Arg::Pos(expr) | Arg::Spread(expr) => {
                expr.render(renderer);
            }
            Arg::Named(named) => named.render(renderer),
        }
    }
}

impl Renderable for Param {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        match self {
            Param::Pos(pat) => pat.render(renderer),
            Param::Named(named) => named.render(renderer),
            Param::Sink(spread) => spread.render(renderer),
        }
    }
}

impl Renderable for ArrayItem {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        match self {
            ArrayItem::Pos(expr) | ArrayItem::Spread(expr) => expr.render(renderer),
        }
    }
}

impl Renderable for DictItem {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
        match self {
            DictItem::Named(named) => named.render(renderer),
            DictItem::Keyed(keyed) => keyed.render(renderer),
            DictItem::Spread(expr) => expr.render(renderer),
        }
    }
}

impl Renderable for Expr {
    fn render(&self, renderer: &mut Renderer) {
        debug!(?self, "rendering");
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
    debug!(?node, "render_args");
    let total = node.children().count();
    let children = node.children();
    for (i, child) in children.enumerate() {
        if let Some(expr) = child.cast::<Expr>() {
            expr.render(renderer);
            if renderer.config().spacing && i + 2 < total {
                renderer.writer.push(", ");
            }
        } else if let Some(named) = child.cast::<Named>() {
            named.render(renderer);
            if renderer.config().spacing && i + 2 < total {
                renderer.writer.push(", ");
            }
        } else if child.kind() == SyntaxKind::Comma {
            if !renderer.config().spacing {
                render_anon(child, renderer);
            }
        } else {
            render_anon(child, renderer);
        }
    }
}

fn render_params(node: &SyntaxNode, renderer: &mut Renderer) {
    debug!(?node, "render_params");
    let total = node.children().count();
    let children = node.children();
    for (i, child) in children.enumerate() {
        if let Some(param) = child.cast::<Param>() {
            param.render(renderer);
            if renderer.config().spacing && i + 2 < total {
                renderer.writer.push(", ");
            }
        } else if child.kind() == SyntaxKind::LeftParen {
            renderer.writer.push("(").inc_indent();
        } else if child.kind() == SyntaxKind::RightParen {
            renderer.writer.dec_indent().push(")");
        } else if child.kind() == SyntaxKind::Comma {
            if !renderer.config().spacing {
                render_anon(child, renderer);
            }
        } else {
            render_anon(child, renderer);
        }
    }
}
