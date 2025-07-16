// ryha-toolchain/ryha/src/semantic.rs

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Variable,
    Function,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    outer: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: SymbolTable) -> Self {
        let mut table = SymbolTable::new();
        table.outer = Some(Box::new(outer));
        table
    }

    pub fn define(&mut self, name: String, symbol: Symbol) {
        self.store.insert(name, symbol);
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        match self.store.get(name) {
            Some(symbol) => Some(symbol.clone()),
            None => match &self.outer {
                Some(outer) => outer.resolve(name),
                None => None,
            },
        }
    }
}

pub struct SemanticAnalyzer {
    table: SymbolTable,
    errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &crate::ast::Program) {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }
    }

    fn analyze_statement(&mut self, statement: &crate::ast::Statement) {
        match statement {
            crate::ast::Statement::Let(name, expr) => {
                self.analyze_expression(expr);
                self.table.define(name.clone(), Symbol::Variable);
            }
            crate::ast::Statement::Return(expr) => {
                self.analyze_expression(expr);
            }
            crate::ast::Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
        }
    }

    fn analyze_expression(&mut self, expression: &crate::ast::Expression) {
        match expression {
            crate::ast::Expression::Ident(name) => {
                if self.table.resolve(name).is_none() {
                    self.errors.push(format!("undefined variable: {}", name));
                }
            }
            crate::ast::Expression::Infix(_, left, right) => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            crate::ast::Expression::Prefix(_, right) => {
                self.analyze_expression(right);
            }
            crate::ast::Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                self.analyze_expression(condition);
                self.analyze_statements(consequence);
                if let Some(alt) = alternative {
                    self.analyze_statements(alt);
                }
            }
            crate::ast::Expression::Function { params, body } => {
                let mut new_table = SymbolTable::new_enclosed(self.table.clone());
                for param in params {
                    new_table.define(param.clone(), Symbol::Variable);
                }
                let old_table = self.table.clone();
                self.table = new_table;
                self.analyze_statements(body);
                self.table = old_table;
            }
            _ => {}
        }
    }

    fn analyze_statements(&mut self, statements: &[crate::ast::Statement]) {
        let mut new_table = SymbolTable::new_enclosed(self.table.clone());
        let old_table = self.table.clone();
        self.table = new_table;
        for statement in statements {
            self.analyze_statement(statement);
        }
        self.table = old_table;
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}
