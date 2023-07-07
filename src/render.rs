use log::debug;
use typst::syntax::{ast::*, LinkedNode, SyntaxKind};

use crate::writer::Writer;

pub struct Renderer<'a> {
    pub writer: Writer<'a>,
}

impl<'a> Renderer<'a> {
    pub fn render(&mut self, node: LinkedNode) {
        debug!("render: {:?}", node);
        match node.kind() {
            SyntaxKind::CodeBlock => self.render_code_block(node.cast().unwrap()),
            SyntaxKind::Markup => self.render_markup(node.cast().unwrap()),
            kind => todo!("Render {:?}", kind),
        }
    }

    fn render_markup(&mut self, node: Markup) {
        debug!("render_markup: {:?}", node);
        for expr in node.exprs() {
            if expr.hashtag() {
                self.writer.push("#");
            }
            self.render_expr(expr);
        }
    }

    fn render_code_block(&mut self, node: CodeBlock) {
        debug!("render_code_block: {:?}", node);
        self.writer.push("{");
        self.render_code(node.body());
        self.writer.push("}");
    }

    fn render_code(&mut self, node: Code) {
        debug!("render_code: {:?}", node);
        let total = node.exprs().count();
        let multiline = total > 1;
        self.writer.inc_indent();
        if multiline {
            self.writer.newline_with_indent();
        }
        for (i, expr) in node.exprs().enumerate() {
            self.render_expr(expr);
            if i + 1 == total {
                // last element
                self.writer.dec_indent();
            }
            if multiline {
                self.writer.newline_with_indent();
            }
        }
    }

    fn render_expr(&mut self, node: Expr) {
        debug!("render_expr: {:?}", node);
        match node {
            Expr::Text(text) => self.render_text(text),
            Expr::Space(space) => self.render_space(space),
            Expr::Linebreak(_) => self.render_linebreak(),
            Expr::Parbreak(parbreak) => self.render_parbreak(parbreak),
            Expr::Escape(escape) => self.render_escape(escape),
            Expr::Shorthand(shorthand) => self.render_shorthand(shorthand),
            Expr::SmartQuote(quote) => self.render_smart_quote(quote),
            Expr::Strong(strong) => self.render_strong(strong),
            Expr::Emph(emph) => self.render_emph(emph),
            Expr::Raw(raw) => self.render_raw(raw),
            Expr::Link(link) => self.render_link(link),
            Expr::Label(label) => self.render_label(label),
            Expr::Ref(reff) => self.render_ref(reff),
            Expr::Heading(heading) => self.render_heading(heading),
            Expr::List(list) => self.render_list(list),
            Expr::Enum(en) => self.render_enum(en),
            Expr::Term(term) => self.render_term(term),
            Expr::Equation(eq) => self.render_equation(eq),
            Expr::Math(m) => self.render_math(m),
            Expr::MathIdent(mi) => self.render_math_ident(mi),
            Expr::MathAlignPoint(map) => self.render_math_align_point(map),
            Expr::MathDelimited(md) => self.render_math_delimited(md),
            Expr::MathAttach(ma) => self.render_math_attach(ma),
            Expr::MathFrac(mf) => self.render_math_frac(mf),
            Expr::MathRoot(mr) => self.render_math_root(mr),
            Expr::Ident(ident) => self.render_ident(ident),
            Expr::None(_) => self.render_none(),
            Expr::Auto(_) => self.render_auto(),
            Expr::Bool(b) => self.render_bool(b),
            Expr::Int(int) => self.render_int(int),
            Expr::Float(f) => self.render_float(f),
            Expr::Numeric(n) => self.render_numeric(n),
            Expr::Str(string) => self.render_string(string),
            Expr::Code(code) => self.render_code_block(code),
            Expr::Content(content) => self.render_content_block(content),
            Expr::Parenthesized(p) => self.render_parenthesized(p),
            Expr::Array(a) => self.render_array(a),
            Expr::Dict(d) => self.render_dict(d),
            Expr::Unary(unary) => self.render_unary(unary),
            Expr::Binary(binary) => self.render_binary(binary),
            Expr::FieldAccess(field_access) => self.render_field_access(field_access),
            Expr::FuncCall(func_call) => self.render_func_call(func_call),
            Expr::Closure(c) => self.render_closure(c),
            Expr::Let(expr) => self.render_let(expr),
            Expr::DestructAssign(da) => self.render_destruct_assign(da),
            Expr::Set(s) => self.render_set(s),
            Expr::Show(show) => self.render_show(show),
            Expr::Conditional(c) => self.render_conditional(c),
            Expr::While(w) => self.render_while(w),
            Expr::For(f) => self.render_for(f),
            Expr::Import(import) => self.render_import(import),
            Expr::Include(include) => self.render_include(include),
            Expr::Break(_) => self.render_break(),
            Expr::Continue(_) => self.render_continue(),
            Expr::Return(ret) => self.render_return(ret),
        }
    }

    fn render_let(&mut self, node: LetBinding) {
        debug!("render_let: {:?}", node);
        self.writer.push("let ");
        match node.kind() {
            LetBindingKind::Normal(pattern) => {
                self.render_pattern(pattern);
            }
            LetBindingKind::Closure(closure) => {
                self.render_ident(closure);
            }
        }
        self.writer.push(" = ");
        if let Some(expr) = node.init() {
            self.render_expr(expr);
        }
    }

    fn render_pattern(&mut self, node: Pattern) {
        debug!("render_pattern: {:?}", node);
        match node {
            Pattern::Normal(expr) => self.render_expr(expr),
            Pattern::Placeholder(_) => self.render_underscore(),
            Pattern::Destructuring(destructuring) => self.render_destructuring(destructuring),
        }
    }

    fn render_underscore(&mut self) {
        debug!("render_underscore");
        self.writer.push("_");
    }

    fn render_destructuring(&mut self, node: Destructuring) {
        debug!("render_destructuring: {:?}", node);
        for ident in node.idents() {
            self.render_ident(ident);
        }
    }

    fn render_ident(&mut self, node: Ident) {
        debug!("render_ident: {:?}", node);
        self.writer.push(node.as_str());
    }

    fn render_int(&mut self, node: Int) {
        debug!("render_int: {:?}", node);
        self.writer.push(&node.get().to_string());
    }

    fn render_func_call(&mut self, node: FuncCall) {
        debug!("render_func_call: {:?}", node);
        self.render_expr(node.callee());
        self.render_args(node.args());
    }

    fn render_args(&mut self, node: Args) {
        debug!("render_args: {:?}", node);
        let total = node.items().count();
        self.writer.push("(");
        self.writer.inc_indent();
        let multiline = total > 1;
        if multiline {
            self.writer.newline_with_indent();
        }
        for (i, item) in node.items().enumerate() {
            self.render_arg(item);
            if multiline {
                self.writer.push(",");
            }
            if i + 1 == total {
                // last element
                self.writer.dec_indent();
            } else if !multiline {
                self.writer.push(" ");
            }
            if multiline {
                self.writer.newline_with_indent();
            }
        }
        self.writer.push(")");
    }

    fn render_arg(&mut self, node: Arg) {
        debug!("render_arg: {:?}", node);
        match node {
            Arg::Pos(expr) => self.render_expr(expr),
            Arg::Named(named) => self.render_named(named),
            Arg::Spread(expr) => self.render_expr(expr),
        }
    }

    fn render_named(&mut self, node: Named) {
        debug!("render_named: {:?}", node);
        self.render_ident(node.name());
        self.writer.push(": ");
        self.render_expr(node.expr());
    }

    fn render_space(&mut self, node: Space) {
        debug!("render_space: {:?}", node);
        self.writer.push(node.as_untyped().text());
    }

    fn render_text(&mut self, node: Text) {
        debug!("render_text: {:?}", node);
        self.writer.push(node.get());
    }

    fn render_string(&mut self, node: Str) {
        debug!("render_string: {:?}", node);
        self.writer.push("\"").push(&node.get()).push("\"");
    }

    fn render_strong(&mut self, node: Strong) {
        debug!("render_strong: {:?}", node);
        self.writer.push("*");
        self.render_markup(node.body());
        self.writer.push("*");
    }

    fn render_emph(&mut self, node: Emph) {
        debug!("render_emph: {:?}", node);
        self.writer.push("_");
        self.render_markup(node.body());
        self.writer.push("_");
    }

    fn render_parbreak(&mut self, node: Parbreak) {
        debug!("render_parbreak: {:?}", node);
        self.writer.newline().newline();
    }

    fn render_content_block(&mut self, node: ContentBlock) {
        debug!("render_content_block: {:?}", node);
        self.writer.push("[");
        self.render_markup(node.body());
        self.writer.push("]");
    }

    fn render_import(&mut self, node: ModuleImport) {
        debug!("render_import: {:?}", node);
        self.writer.push("import ");
        self.render_expr(node.source());
        if let Some(imports) = node.imports() {
            self.writer.push(": ");
            self.render_imports(imports);
        }
        self.writer.newline();
    }

    fn render_imports(&mut self, node: Imports) {
        debug!("render_imports: {:?}", node);
        match node {
            Imports::Wildcard => {
                self.writer.push("*");
            }
            Imports::Items(items) => {
                for item in items {
                    self.render_ident(item);
                }
            }
        }
    }

    fn render_show(&mut self, node: ShowRule) {
        debug!("render_show: {:?}", node);
        self.writer.push("show");
        if let Some(selector) = node.selector() {
            self.writer.push(" ");
            self.render_expr(selector);
        } else {
            self.writer.push(": ");
        }
        self.render_expr(node.transform());
    }

    fn render_field_access(&mut self, node: FieldAccess) {
        debug!("render_field_access: {:?}", node);
        self.render_expr(node.target());
        self.writer.push(".");
        self.render_ident(node.field());
    }

    fn render_linebreak(&mut self) {
        debug!("render_linebreak");
        self.writer.push("\\");
    }

    fn render_escape(&mut self, node: Escape) {
        debug!("render_escape: {:?}", node);
        self.writer.push(node.as_untyped().text());
    }

    fn render_smart_quote(&mut self, node: SmartQuote) {
        debug!("render_smart_quote: {:?}", node);
        if node.double() {
            self.writer.push("\"");
        } else {
            self.writer.push("'");
        }
    }

    fn render_link(&mut self, node: Link) {
        debug!("render_link: {:?}", node);
        self.writer.push(&node.get());
    }

    fn render_label(&mut self, node: Label) {
        debug!("render_label: {:?}", node);
        self.writer.push("<").push(node.get()).push(">");
    }

    fn render_none(&mut self) {
        debug!("render_none");
        self.writer.push("none");
    }

    fn render_auto(&mut self) {
        debug!("render_auto");
        self.writer.push("auto");
    }

    fn render_bool(&mut self, node: Bool) {
        debug!("render_bool: {:?}", node);
        if node.get() {
            self.writer.push("true");
        } else {
            self.writer.push("false");
        }
    }

    fn render_float(&mut self, node: Float) {
        debug!("render_float: {:?}", node);
        self.writer.push(&node.get().to_string());
    }

    fn render_shorthand(&mut self, node: Shorthand) {
        debug!("render_shorthand: {:?}", node);
        self.writer.push(&node.get().to_string());
    }

    fn render_heading(&mut self, node: Heading) {
        debug!("render_heading: {:?}", node);
        self.writer.push(&"=".repeat(node.level().get()));
        self.writer.push(" ");
        self.render_markup(node.body())
    }

    fn render_unary(&mut self, node: Unary) {
        debug!("render_unary: {:?}", node);
        self.writer.push(node.op().as_str());
        self.render_expr(node.expr());
    }

    fn render_binary(&mut self, node: Binary) {
        debug!("render_binary: {:?}", node);
        self.render_expr(node.lhs());
        self.writer.push(node.op().as_str());
        self.render_expr(node.rhs());
    }

    fn render_include(&mut self, node: ModuleInclude) {
        debug!("render_include: {:?}", node);
        self.writer.push("include \"");
        self.render_expr(node.source());
        self.writer.push("\"");
    }

    fn render_break(&mut self) {
        debug!("render_break");
        self.writer.push("break");
    }

    fn render_continue(&mut self) {
        debug!("render_continue");
        self.writer.push("continue");
    }

    fn render_return(&mut self, node: FuncReturn) {
        debug!("render_return: {:?}", node);
        self.writer.push("return");
        if let Some(body) = node.body() {
            self.render_expr(body);
        }
    }

    fn render_raw(&mut self, node: Raw) {
        debug!("render_raw: {:?}", node);
        let end_str = if node.block() { "`" } else { "```" };
        self.writer.push(end_str);
        if let Some(lang) = node.lang() {
            self.writer.push(lang);
        }
        self.writer.push(&node.text());
        self.writer.push(end_str);
    }

    fn render_ref(&mut self, node: Ref) {
        debug!("render_ref: {:?}", node);
        self.writer.push("@");
        self.writer.push(node.target());
        if let Some(supp) = node.supplement() {
            self.render_content_block(supp);
        }
    }

    fn render_list(&mut self, node: ListItem) {
        debug!("render_list: {:?}", node);
        self.writer.push("- ");
        self.render_markup(node.body());
    }

    fn render_enum(&mut self, node: EnumItem) {
        debug!("render_enum: {:?}", node);
        if let Some(number) = node.number() {
            self.writer.push(&number.to_string());
        } else {
            self.writer.push("+");
        }
        self.writer.push(" ");
        self.render_markup(node.body());
    }

    fn render_term(&mut self, node: TermItem) {
        debug!("render_term: {:?}", node);
        self.writer.push("/ ");
        self.render_markup(node.term());
        self.writer.push(": ");
        self.render_markup(node.description());
    }

    fn render_equation(&mut self, node: Equation) {
        debug!("render_equation: {:?}", node);
        self.writer.push("$");
        if node.block() {
            self.writer.push(" ");
        }
        self.render_math(node.body());
        if node.block() {
            self.writer.push(" ");
        }
        self.writer.push("$");
    }

    fn render_math(&mut self, node: Math) {
        debug!("render_math: {:?}", node);
        for expr in node.exprs() {
            self.render_expr(expr)
        }
    }

    fn render_math_ident(&mut self, node: MathIdent) {
        debug!("render_math_ident: {:?}", node);
        self.writer.push(node.as_str());
    }

    fn render_math_align_point(&mut self, node: MathAlignPoint) {
        debug!("render_math_align_point: {:?}", node);
        self.writer.push("&");
    }

    fn render_math_delimited(&mut self, node: MathDelimited) {
        debug!("render_math_delimited: {:?}", node);
        self.render_expr(node.open());
        self.render_math(node.body());
        self.render_expr(node.close());
    }

    fn render_math_attach(&mut self, node: MathAttach) {
        debug!("render_math_attach: {:?}", node);
        self.render_expr(node.base());
        if let Some(bottom) = node.bottom() {
            self.render_expr(bottom);
        }
        if let Some(top) = node.top() {
            self.render_expr(top);
        }
    }

    fn render_math_frac(&mut self, node: MathFrac) {
        debug!("render_math_frac: {:?}", node);
        self.render_expr(node.num());
        self.writer.push("/");
        self.render_expr(node.denom());
    }

    fn render_math_root(&mut self, node: MathRoot) {
        debug!("render_math_root: {:?}", node);
        let sym = match node.index() {
            Some(4) => "∜",
            Some(3) => "∛",
            Some(2) => "√",
            _ => "",
        };
        self.writer.push(sym);
        self.render_expr(node.radicand());
    }

    fn render_numeric(&mut self, node: Numeric) {
        debug!("render_numeric: {:?}", node);
        self.writer.push(node.as_untyped().text());
    }

    fn render_parenthesized(&mut self, node: Parenthesized) {
        debug!("render_parenthesized: {:?}", node);
        self.writer.push("(");
        self.render_expr(node.expr());
        self.writer.push(")");
    }

    fn render_array(&mut self, node: Array) {
        debug!("render_array: {:?}", node);
        self.writer.push("(");
        for item in node.items() {
            match item {
                ArrayItem::Pos(expr) => self.render_expr(expr),
                ArrayItem::Spread(expr) => self.render_expr(expr),
            }
        }
        self.writer.push(")");
    }

    fn render_dict(&mut self, node: Dict) {
        debug!("render_dict: {:?}", node);
        self.writer.push("(");
        for item in node.items() {
            match item {
                DictItem::Named(n) => self.render_named(n),
                DictItem::Keyed(k) => self.render_keyed(k),
                DictItem::Spread(e) => self.render_expr(e),
            }
        }
        self.writer.push(")");
    }

    fn render_keyed(&mut self, node: Keyed) {
        debug!("render_keyed: {:?}", node);
        self.render_string(node.key());
        self.writer.push(": ");
        self.render_expr(node.expr());
    }

    fn render_closure(&mut self, node: Closure) {
        debug!("render_closure: {:?}", node);
        self.render_params(node.params());
        self.writer.push(" => ");
        self.render_expr(node.body());
    }

    fn render_params(&mut self, node: Params) {
        debug!("render_params: {:?}", node);
        self.writer.push("(");
        for child in node.children() {
            self.render_param(child);
            self.writer.push(",");
        }
        self.writer.push(")");
    }

    fn render_param(&mut self, node: Param) {
        debug!("render_param: {:?}", node);
        match node {
            Param::Pos(p) => self.render_pattern(p),
            Param::Named(n) => self.render_named(n),
            Param::Sink(s) => self.render_spread(s),
        }
    }

    fn render_spread(&mut self, node: Spread) {
        debug!("render_spread: {:?}", node);
        self.writer.push("..");
        if let Some(name) = node.name() {
            self.render_ident(name);
        }
        if let Some(expr) = node.expr() {
            self.render_expr(expr);
        }
    }

    fn render_destruct_assign(&mut self, node: DestructAssignment) {
        debug!("render_destruct_assign: {:?}", node);
        self.render_pattern(node.pattern());
        self.writer.push(" = ");
        self.render_expr(node.value());
    }

    fn render_set(&mut self, node: SetRule) {
        debug!("render_set: {:?}", node);
        self.writer.push("set ");
        self.render_expr(node.target());
        self.render_args(node.args());
        if let Some(expr) = node.condition() {
            self.render_expr(expr);
        }
    }

    fn render_conditional(&mut self, node: Conditional) {
        debug!("render_conditional: {:?}", node);
        self.writer.push("if ");
        self.render_expr(node.condition());
        self.writer.push(" { ");
        self.render_expr(node.if_body());
        self.writer.push(" }");
        if let Some(els) = node.else_body() {
            self.writer.push(" else { ");
            self.render_expr(els);
            self.writer.push(" }");
        }
    }

    fn render_while(&mut self, node: WhileLoop) {
        debug!("render_while: {:?}", node);
        self.writer.push("while ");
        self.render_expr(node.condition());
        self.writer.push(" { ");
        self.render_expr(node.body());
        self.writer.push(" }");
    }

    fn render_for(&mut self, node: ForLoop) {
        debug!("render_for: {:?}", node);
        self.writer.push("for ");
        self.render_pattern(node.pattern());
        self.writer.push(" in ");
        self.render_expr(node.iter());
        self.writer.push(" { ");
        self.render_expr(node.body());
        self.writer.push(" }");
    }
}
