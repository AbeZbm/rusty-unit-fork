// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum Java {
    End = 0,
    Identifier = 1,
    DecimalIntegerLiteral = 2,
    HexIntegerLiteral = 3,
    OctalIntegerLiteral = 4,
    BinaryIntegerLiteral = 5,
    DecimalFloatingPointLiteral = 6,
    HexFloatingPointLiteral = 7,
    True = 8,
    False = 9,
    CharacterLiteral = 10,
    StringLiteral = 11,
    NullLiteral = 12,
    LPAREN = 13,
    AMP = 14,
    RPAREN = 15,
    EQ = 16,
    PLUSEQ = 17,
    DASHEQ = 18,
    STAREQ = 19,
    SLASHEQ = 20,
    AMPEQ = 21,
    PIPEEQ = 22,
    CARETEQ = 23,
    PERCENTEQ = 24,
    LTLTEQ = 25,
    GTGTEQ = 26,
    GTGTGTEQ = 27,
    GT = 28,
    LT = 29,
    EQEQ = 30,
    GTEQ = 31,
    LTEQ = 32,
    BANGEQ = 33,
    AMPAMP = 34,
    PIPEPIPE = 35,
    PLUS = 36,
    DASH = 37,
    STAR = 38,
    SLASH = 39,
    PIPE = 40,
    CARET = 41,
    PERCENT = 42,
    LTLT = 43,
    GTGT = 44,
    GTGTGT = 45,
    Instanceof = 46,
    DASHGT = 47,
    COMMA = 48,
    QMARK = 49,
    COLON = 50,
    BANG = 51,
    TILDE = 52,
    PLUSPLUS = 53,
    DASHDASH = 54,
    New = 55,
    LBRACK = 56,
    RBRACK = 57,
    DOT = 58,
    Class = 59,
    COLONCOLON = 60,
    Extends = 61,
    SEMI = 62,
    LBRACE = 63,
    RBRACE = 64,
    Assert = 65,
    Switch = 66,
    Case = 67,
    Default = 68,
    Do = 69,
    While = 70,
    Break = 71,
    Continue = 72,
    Return = 73,
    Synchronized = 74,
    Throw = 75,
    Try = 76,
    Catch = 77,
    Finally = 78,
    If = 79,
    Else = 80,
    For = 81,
    AT = 82,
    Open = 83,
    Module = 84,
    Requires = 85,
    Exports = 86,
    To = 87,
    Opens = 88,
    Uses = 89,
    Provides = 90,
    With = 91,
    Transitive = 92,
    Static = 93,
    Package = 94,
    Import = 95,
    Enum = 96,
    Public = 97,
    Protected = 98,
    Private = 99,
    Abstract = 100,
    Final = 101,
    Strictfp = 102,
    Native = 103,
    Transient = 104,
    Volatile = 105,
    Implements = 106,
    ATinterface = 107,
    Interface = 108,
    Byte = 109,
    Short = 110,
    Int = 111,
    Long = 112,
    Char = 113,
    Float = 114,
    Double = 115,
    BooleanType = 116,
    VoidType = 117,
    DOTDOTDOT = 118,
    Throws2 = 119,
    This = 120,
    Super = 121,
    Comment = 122,
    Program = 123,
    Literal = 124,
    Expression = 125,
    CastExpression = 126,
    AssignmentExpression = 127,
    BinaryExpression = 128,
    InstanceofExpression = 129,
    LambdaExpression = 130,
    InferredParameters = 131,
    TernaryExpression = 132,
    UnaryExpression = 133,
    UpdateExpression = 134,
    PrimaryExpression = 135,
    ArrayCreationExpression = 136,
    DimensionsExpr = 137,
    ParenthesizedExpression = 138,
    ClassLiteral = 139,
    ObjectCreationExpression = 140,
    UnqualifiedObjectCreationExpression = 141,
    FieldAccess = 142,
    ArrayAccess = 143,
    MethodInvocation = 144,
    ArgumentList = 145,
    MethodReference = 146,
    TypeArguments = 147,
    Wildcard = 148,
    WildcardBounds = 149,
    Dimensions = 150,
    Statement = 151,
    Block = 152,
    ExpressionStatement = 153,
    LabeledStatement = 154,
    AssertStatement = 155,
    SwitchStatement = 156,
    SwitchBlock = 157,
    SwitchLabel = 158,
    DoStatement = 159,
    BreakStatement = 160,
    ContinueStatement = 161,
    ReturnStatement = 162,
    SynchronizedStatement = 163,
    ThrowStatement = 164,
    TryStatement = 165,
    CatchClause = 166,
    CatchFormalParameter = 167,
    CatchType = 168,
    FinallyClause = 169,
    TryWithResourcesStatement = 170,
    ResourceSpecification = 171,
    Resource = 172,
    IfStatement = 173,
    WhileStatement = 174,
    ForStatement = 175,
    EnhancedForStatement = 176,
    Annotation = 177,
    MarkerAnnotation = 178,
    Annotation2 = 179,
    AnnotationArgumentList = 180,
    ElementValuePair = 181,
    ElementValue = 182,
    ElementValueArrayInitializer = 183,
    Declaration = 184,
    ModuleDeclaration = 185,
    ModuleBody = 186,
    ModuleDirective = 187,
    RequiresModifier = 188,
    PackageDeclaration = 189,
    ImportDeclaration = 190,
    Asterisk = 191,
    EnumDeclaration = 192,
    EnumBody = 193,
    EnumBodyDeclarations = 194,
    EnumConstant = 195,
    ClassDeclaration = 196,
    Modifiers = 197,
    TypeParameters = 198,
    TypeParameter = 199,
    TypeBound = 200,
    Superclass = 201,
    SuperInterfaces = 202,
    InterfaceTypeList = 203,
    ClassBody = 204,
    StaticInitializer = 205,
    ConstructorDeclaration = 206,
    ConstructorDeclarator = 207,
    ConstructorBody = 208,
    ExplicitConstructorInvocation = 209,
    ScopedIdentifier = 210,
    FieldDeclaration = 211,
    AnnotationTypeDeclaration = 212,
    AnnotationTypeBody = 213,
    AnnotationTypeElementDeclaration = 214,
    DefaultValue = 215,
    InterfaceDeclaration = 216,
    ExtendsInterfaces = 217,
    InterfaceBody = 218,
    ConstantDeclaration = 219,
    VariableDeclaratorList = 220,
    VariableDeclarator = 221,
    VariableDeclaratorId = 222,
    ArrayInitializer = 223,
    Type = 224,
    UnannotatedType = 225,
    AnnotatedType = 226,
    ScopedTypeIdentifier = 227,
    GenericType = 228,
    ArrayType = 229,
    IntegralType = 230,
    FloatingPointType = 231,
    MethodHeader = 232,
    MethodDeclarator = 233,
    FormalParameters = 234,
    FormalParameter = 235,
    ReceiverParameter = 236,
    SpreadParameter = 237,
    Throws = 238,
    LocalVariableDeclaration = 239,
    MethodDeclaration = 240,
    ProgramRepeat1 = 241,
    CastExpressionRepeat1 = 242,
    InferredParametersRepeat1 = 243,
    ArrayCreationExpressionRepeat1 = 244,
    DimensionsExprRepeat1 = 245,
    ArgumentListRepeat1 = 246,
    TypeArgumentsRepeat1 = 247,
    DimensionsRepeat1 = 248,
    SwitchBlockRepeat1 = 249,
    TryStatementRepeat1 = 250,
    CatchTypeRepeat1 = 251,
    ResourceSpecificationRepeat1 = 252,
    ForStatementRepeat1 = 253,
    ForStatementRepeat2 = 254,
    AnnotationArgumentListRepeat1 = 255,
    ElementValueArrayInitializerRepeat1 = 256,
    ModuleBodyRepeat1 = 257,
    ModuleDirectiveRepeat1 = 258,
    ModuleDirectiveRepeat2 = 259,
    EnumBodyRepeat1 = 260,
    EnumBodyDeclarationsRepeat1 = 261,
    ModifiersRepeat1 = 262,
    TypeParametersRepeat1 = 263,
    TypeBoundRepeat1 = 264,
    InterfaceTypeListRepeat1 = 265,
    AnnotationTypeBodyRepeat1 = 266,
    InterfaceBodyRepeat1 = 267,
    VariableDeclaratorListRepeat1 = 268,
    ArrayInitializerRepeat1 = 269,
    FormalParametersRepeat1 = 270,
    TypeIdentifier = 271,
    Error = 272,
}

impl From<Java> for &'static str {
    #[inline(always)]
    fn from(tok: Java) -> Self {
        match tok {
            Java::End => "end",
            Java::Identifier => "identifier",
            Java::DecimalIntegerLiteral => "decimal_integer_literal",
            Java::HexIntegerLiteral => "hex_integer_literal",
            Java::OctalIntegerLiteral => "octal_integer_literal",
            Java::BinaryIntegerLiteral => "binary_integer_literal",
            Java::DecimalFloatingPointLiteral => "decimal_floating_point_literal",
            Java::HexFloatingPointLiteral => "hex_floating_point_literal",
            Java::True => "true",
            Java::False => "false",
            Java::CharacterLiteral => "character_literal",
            Java::StringLiteral => "string_literal",
            Java::NullLiteral => "null_literal",
            Java::LPAREN => "(",
            Java::AMP => "&",
            Java::RPAREN => ")",
            Java::EQ => "=",
            Java::PLUSEQ => "+=",
            Java::DASHEQ => "-=",
            Java::STAREQ => "*=",
            Java::SLASHEQ => "/=",
            Java::AMPEQ => "&=",
            Java::PIPEEQ => "|=",
            Java::CARETEQ => "^=",
            Java::PERCENTEQ => "%=",
            Java::LTLTEQ => "<<=",
            Java::GTGTEQ => ">>=",
            Java::GTGTGTEQ => ">>>=",
            Java::GT => ">",
            Java::LT => "<",
            Java::EQEQ => "==",
            Java::GTEQ => ">=",
            Java::LTEQ => "<=",
            Java::BANGEQ => "!=",
            Java::AMPAMP => "&&",
            Java::PIPEPIPE => "||",
            Java::PLUS => "+",
            Java::DASH => "-",
            Java::STAR => "*",
            Java::SLASH => "/",
            Java::PIPE => "|",
            Java::CARET => "^",
            Java::PERCENT => "%",
            Java::LTLT => "<<",
            Java::GTGT => ">>",
            Java::GTGTGT => ">>>",
            Java::Instanceof => "instanceof",
            Java::DASHGT => "->",
            Java::COMMA => ",",
            Java::QMARK => "?",
            Java::COLON => ":",
            Java::BANG => "!",
            Java::TILDE => "~",
            Java::PLUSPLUS => "++",
            Java::DASHDASH => "--",
            Java::New => "new",
            Java::LBRACK => "[",
            Java::RBRACK => "]",
            Java::DOT => ".",
            Java::Class => "class",
            Java::COLONCOLON => "::",
            Java::Extends => "extends",
            Java::SEMI => ";",
            Java::LBRACE => "{",
            Java::RBRACE => "}",
            Java::Assert => "assert",
            Java::Switch => "switch",
            Java::Case => "case",
            Java::Default => "default",
            Java::Do => "do",
            Java::While => "while",
            Java::Break => "break",
            Java::Continue => "continue",
            Java::Return => "return",
            Java::Synchronized => "synchronized",
            Java::Throw => "throw",
            Java::Try => "try",
            Java::Catch => "catch",
            Java::Finally => "finally",
            Java::If => "if",
            Java::Else => "else",
            Java::For => "for",
            Java::AT => "@",
            Java::Open => "open",
            Java::Module => "module",
            Java::Requires => "requires",
            Java::Exports => "exports",
            Java::To => "to",
            Java::Opens => "opens",
            Java::Uses => "uses",
            Java::Provides => "provides",
            Java::With => "with",
            Java::Transitive => "transitive",
            Java::Static => "static",
            Java::Package => "package",
            Java::Import => "import",
            Java::Enum => "enum",
            Java::Public => "public",
            Java::Protected => "protected",
            Java::Private => "private",
            Java::Abstract => "abstract",
            Java::Final => "final",
            Java::Strictfp => "strictfp",
            Java::Native => "native",
            Java::Transient => "transient",
            Java::Volatile => "volatile",
            Java::Implements => "implements",
            Java::ATinterface => "@interface",
            Java::Interface => "interface",
            Java::Byte => "byte",
            Java::Short => "short",
            Java::Int => "int",
            Java::Long => "long",
            Java::Char => "char",
            Java::Float => "float",
            Java::Double => "double",
            Java::BooleanType => "boolean_type",
            Java::VoidType => "void_type",
            Java::DOTDOTDOT => "...",
            Java::Throws2 => "throws",
            Java::This => "this",
            Java::Super => "super",
            Java::Comment => "comment",
            Java::Program => "program",
            Java::Literal => "_literal",
            Java::Expression => "expression",
            Java::CastExpression => "cast_expression",
            Java::AssignmentExpression => "assignment_expression",
            Java::BinaryExpression => "binary_expression",
            Java::InstanceofExpression => "instanceof_expression",
            Java::LambdaExpression => "lambda_expression",
            Java::InferredParameters => "inferred_parameters",
            Java::TernaryExpression => "ternary_expression",
            Java::UnaryExpression => "unary_expression",
            Java::UpdateExpression => "update_expression",
            Java::PrimaryExpression => "primary_expression",
            Java::ArrayCreationExpression => "array_creation_expression",
            Java::DimensionsExpr => "dimensions_expr",
            Java::ParenthesizedExpression => "parenthesized_expression",
            Java::ClassLiteral => "class_literal",
            Java::ObjectCreationExpression => "object_creation_expression",
            Java::UnqualifiedObjectCreationExpression => "_unqualified_object_creation_expression",
            Java::FieldAccess => "field_access",
            Java::ArrayAccess => "array_access",
            Java::MethodInvocation => "method_invocation",
            Java::ArgumentList => "argument_list",
            Java::MethodReference => "method_reference",
            Java::TypeArguments => "type_arguments",
            Java::Wildcard => "wildcard",
            Java::WildcardBounds => "_wildcard_bounds",
            Java::Dimensions => "dimensions",
            Java::Statement => "statement",
            Java::Block => "block",
            Java::ExpressionStatement => "expression_statement",
            Java::LabeledStatement => "labeled_statement",
            Java::AssertStatement => "assert_statement",
            Java::SwitchStatement => "switch_statement",
            Java::SwitchBlock => "switch_block",
            Java::SwitchLabel => "switch_label",
            Java::DoStatement => "do_statement",
            Java::BreakStatement => "break_statement",
            Java::ContinueStatement => "continue_statement",
            Java::ReturnStatement => "return_statement",
            Java::SynchronizedStatement => "synchronized_statement",
            Java::ThrowStatement => "throw_statement",
            Java::TryStatement => "try_statement",
            Java::CatchClause => "catch_clause",
            Java::CatchFormalParameter => "catch_formal_parameter",
            Java::CatchType => "catch_type",
            Java::FinallyClause => "finally_clause",
            Java::TryWithResourcesStatement => "try_with_resources_statement",
            Java::ResourceSpecification => "resource_specification",
            Java::Resource => "resource",
            Java::IfStatement => "if_statement",
            Java::WhileStatement => "while_statement",
            Java::ForStatement => "for_statement",
            Java::EnhancedForStatement => "enhanced_for_statement",
            Java::Annotation => "_annotation",
            Java::MarkerAnnotation => "marker_annotation",
            Java::Annotation2 => "annotation",
            Java::AnnotationArgumentList => "annotation_argument_list",
            Java::ElementValuePair => "element_value_pair",
            Java::ElementValue => "_element_value",
            Java::ElementValueArrayInitializer => "element_value_array_initializer",
            Java::Declaration => "declaration",
            Java::ModuleDeclaration => "module_declaration",
            Java::ModuleBody => "module_body",
            Java::ModuleDirective => "module_directive",
            Java::RequiresModifier => "requires_modifier",
            Java::PackageDeclaration => "package_declaration",
            Java::ImportDeclaration => "import_declaration",
            Java::Asterisk => "asterisk",
            Java::EnumDeclaration => "enum_declaration",
            Java::EnumBody => "enum_body",
            Java::EnumBodyDeclarations => "enum_body_declarations",
            Java::EnumConstant => "enum_constant",
            Java::ClassDeclaration => "class_declaration",
            Java::Modifiers => "modifiers",
            Java::TypeParameters => "type_parameters",
            Java::TypeParameter => "type_parameter",
            Java::TypeBound => "type_bound",
            Java::Superclass => "superclass",
            Java::SuperInterfaces => "super_interfaces",
            Java::InterfaceTypeList => "interface_type_list",
            Java::ClassBody => "class_body",
            Java::StaticInitializer => "static_initializer",
            Java::ConstructorDeclaration => "constructor_declaration",
            Java::ConstructorDeclarator => "_constructor_declarator",
            Java::ConstructorBody => "constructor_body",
            Java::ExplicitConstructorInvocation => "explicit_constructor_invocation",
            Java::ScopedIdentifier => "scoped_identifier",
            Java::FieldDeclaration => "field_declaration",
            Java::AnnotationTypeDeclaration => "annotation_type_declaration",
            Java::AnnotationTypeBody => "annotation_type_body",
            Java::AnnotationTypeElementDeclaration => "annotation_type_element_declaration",
            Java::DefaultValue => "_default_value",
            Java::InterfaceDeclaration => "interface_declaration",
            Java::ExtendsInterfaces => "extends_interfaces",
            Java::InterfaceBody => "interface_body",
            Java::ConstantDeclaration => "constant_declaration",
            Java::VariableDeclaratorList => "_variable_declarator_list",
            Java::VariableDeclarator => "variable_declarator",
            Java::VariableDeclaratorId => "_variable_declarator_id",
            Java::ArrayInitializer => "array_initializer",
            Java::Type => "_type",
            Java::UnannotatedType => "_unannotated_type",
            Java::AnnotatedType => "annotated_type",
            Java::ScopedTypeIdentifier => "scoped_type_identifier",
            Java::GenericType => "generic_type",
            Java::ArrayType => "array_type",
            Java::IntegralType => "integral_type",
            Java::FloatingPointType => "floating_point_type",
            Java::MethodHeader => "_method_header",
            Java::MethodDeclarator => "_method_declarator",
            Java::FormalParameters => "formal_parameters",
            Java::FormalParameter => "formal_parameter",
            Java::ReceiverParameter => "receiver_parameter",
            Java::SpreadParameter => "spread_parameter",
            Java::Throws => "throws",
            Java::LocalVariableDeclaration => "local_variable_declaration",
            Java::MethodDeclaration => "method_declaration",
            Java::ProgramRepeat1 => "program_repeat1",
            Java::CastExpressionRepeat1 => "cast_expression_repeat1",
            Java::InferredParametersRepeat1 => "inferred_parameters_repeat1",
            Java::ArrayCreationExpressionRepeat1 => "array_creation_expression_repeat1",
            Java::DimensionsExprRepeat1 => "dimensions_expr_repeat1",
            Java::ArgumentListRepeat1 => "argument_list_repeat1",
            Java::TypeArgumentsRepeat1 => "type_arguments_repeat1",
            Java::DimensionsRepeat1 => "dimensions_repeat1",
            Java::SwitchBlockRepeat1 => "switch_block_repeat1",
            Java::TryStatementRepeat1 => "try_statement_repeat1",
            Java::CatchTypeRepeat1 => "catch_type_repeat1",
            Java::ResourceSpecificationRepeat1 => "resource_specification_repeat1",
            Java::ForStatementRepeat1 => "for_statement_repeat1",
            Java::ForStatementRepeat2 => "for_statement_repeat2",
            Java::AnnotationArgumentListRepeat1 => "annotation_argument_list_repeat1",
            Java::ElementValueArrayInitializerRepeat1 => "element_value_array_initializer_repeat1",
            Java::ModuleBodyRepeat1 => "module_body_repeat1",
            Java::ModuleDirectiveRepeat1 => "module_directive_repeat1",
            Java::ModuleDirectiveRepeat2 => "module_directive_repeat2",
            Java::EnumBodyRepeat1 => "enum_body_repeat1",
            Java::EnumBodyDeclarationsRepeat1 => "enum_body_declarations_repeat1",
            Java::ModifiersRepeat1 => "modifiers_repeat1",
            Java::TypeParametersRepeat1 => "type_parameters_repeat1",
            Java::TypeBoundRepeat1 => "type_bound_repeat1",
            Java::InterfaceTypeListRepeat1 => "interface_type_list_repeat1",
            Java::AnnotationTypeBodyRepeat1 => "annotation_type_body_repeat1",
            Java::InterfaceBodyRepeat1 => "interface_body_repeat1",
            Java::VariableDeclaratorListRepeat1 => "_variable_declarator_list_repeat1",
            Java::ArrayInitializerRepeat1 => "array_initializer_repeat1",
            Java::FormalParametersRepeat1 => "formal_parameters_repeat1",
            Java::TypeIdentifier => "type_identifier",
            Java::Error => "ERROR",
        }
    }
}

impl From<u16> for Java {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Java == u16
impl PartialEq<u16> for Java {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Java::from(*x)
    }
}

// u16 == Java
impl PartialEq<Java> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Java) -> bool {
        *x == *self
    }
}
